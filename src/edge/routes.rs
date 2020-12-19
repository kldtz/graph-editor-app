use crate::api_error::GraphError;
use crate::edge::{Edge, EdgeInit, EdgePatch};
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;

#[get("/edges/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, GraphError> {
    let edge = Edge::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(edge))
}

#[post("/edges")]
async fn create(edge: web::Json<EdgeInit>) -> Result<HttpResponse, GraphError> {
    let edge = Edge::create(edge.into_inner())?;
    Ok(HttpResponse::Created().json(edge))
}

#[patch("/edges/{id}")]
async fn patch(id: web::Path<i32>, edge: web::Json<EdgePatch>) -> Result<HttpResponse, GraphError> {
    let edge = Edge::patch(id.into_inner(), edge.into_inner())?;
    Ok(HttpResponse::Ok().json(edge))
}

#[delete("/edges/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, GraphError> {
    let num_deleted = Edge::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(create);
    cfg.service(patch);
    cfg.service(delete);
}
