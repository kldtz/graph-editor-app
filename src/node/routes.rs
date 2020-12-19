use crate::api_error::GraphError;
use crate::node::{Node, NodeInit, NodePatch};
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;

#[get("/nodes/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, GraphError> {
    let node = Node::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(node))
}

#[post("/nodes")]
async fn create(node: web::Json<NodeInit>) -> Result<HttpResponse, GraphError> {
    let node = Node::create(node.into_inner())?;
    Ok(HttpResponse::Created().json(node))
}

#[patch("/nodes/{id}")]
async fn patch(id: web::Path<i32>, node: web::Json<NodePatch>) -> Result<HttpResponse, GraphError> {
    let node = Node::patch(id.into_inner(), node.into_inner())?;
    Ok(HttpResponse::Ok().json(node))
}

#[delete("/nodes/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, GraphError> {
    let num_deleted = Node::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(create);
    cfg.service(patch);
    cfg.service(delete);
}
