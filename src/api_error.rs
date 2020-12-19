use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct GraphError {
    pub status_code: u16,
    pub message: String,
}

impl GraphError {
    pub fn new(status_code: u16, message: String) -> GraphError {
        GraphError {
            status_code,
            message,
        }
    }
}

// implement fmt:Display trait for GraphError
impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for GraphError {
    fn from(error: DieselError) -> GraphError {
        match error {
            DieselError::DatabaseError(_, err) => GraphError::new(409, err.message().to_string()),
            DieselError::NotFound => GraphError::new(404, "Record not found".to_string()),
            err => GraphError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl From<askama::Error> for GraphError {
    fn from(error: askama::Error) -> GraphError {
        match error {
            err => GraphError::new(500, format!("Render error: {}", err)),
        }
    }
}

impl ResponseError for GraphError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                // log error, hide message from user
                error!("{}", self.message);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
