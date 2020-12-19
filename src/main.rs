#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate serde_json;

use actix_files as fs;
use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use std::env;

use askama::Template;

mod api_error;
mod db;
mod edge;
mod graph;
mod node;
mod schema;

#[get("/")]
async fn index() -> Result<HttpResponse, api_error::GraphError> {
    Ok(HttpResponse::MovedPermanently()
        .header("Location", "/graphs")
        .finish())
}

#[get("/favicon.ico")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").expect("HOST variable not set");
    let port = env::var("PORT").expect("PORT variable not set");

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .service(favicon)
            .service(fs::Files::new("/static", "./static"))
            .service(index)
            .configure(graph::init_routes)
            .configure(node::init_routes)
            .configure(edge::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    error: &'a str,
    status_code: &'a str,
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };
    let status = res.status();
    let template = ErrorTemplate {
        error: error,
        status_code: status.as_str(),
    };
    let body = template.render();
    match body {
        Ok(body) => Response::build(res.status())
            .content_type("text/html")
            .body(body),
        Err(_) => fallback(error),
    }
}
