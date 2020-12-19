use crate::api_error::GraphError;
use crate::graph::{ExtendedGraph, Graph, GraphInit, GraphPatch};
use actix_web::{delete, get, post, web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "graphs/index.html")]
struct GraphsTemplate<'a> {
    title: &'a str,
    graphs: Vec<Graph>,
}

#[get("/graphs")]
async fn find_all() -> Result<HttpResponse, GraphError> {
    let graphs = Graph::find_all()?;
    let template = GraphsTemplate {
        title: "Graphs",
        graphs: graphs,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Template)]
#[template(path = "graphs/graph.html")]
struct GraphTemplate<'a> {
    title: &'a str,
    graph_id: i32,
}

#[get("/graphs/find/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, GraphError> {
    let graph = ExtendedGraph::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(graph))
}

#[get("/graphs/show/{id}")]
async fn show(id: web::Path<i32>) -> Result<HttpResponse, GraphError> {
    let graph_id = id.into_inner();
    let graph = Graph::find(graph_id)?;
    let template = GraphTemplate {
        title: &graph.graph_label,
        graph_id: graph_id,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Template)]
#[template(path = "graphs/new.html")]
struct NewGraphTemplate<'a> {
    title: &'a str,
}

#[get("/graphs/new")]
async fn new() -> Result<HttpResponse, GraphError> {
    let template = NewGraphTemplate { title: "New Graph" };
    let body = template.render()?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/graphs")]
async fn create(graph: web::Form<GraphInit>) -> Result<HttpResponse, GraphError> {
    Graph::create(graph.into_inner())?;
    Ok(HttpResponse::SeeOther()
        .header("Location", "/graphs")
        .finish())
}

#[patch("/graphs/{id}")]
async fn patch(
    id: web::Path<i32>,
    graph: web::Json<GraphPatch>,
) -> Result<HttpResponse, GraphError> {
    let id = id.into_inner();
    let graph = Graph::patch(id, graph.into_inner())?;
    Ok(HttpResponse::Ok().json(graph))
}

#[delete("/graphs/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, GraphError> {
    Graph::delete(id.into_inner())?;
    Ok(HttpResponse::SeeOther()
        .header("Location", "/graphs")
        .finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(show);
    cfg.service(new);
    cfg.service(create);
    cfg.service(patch);
    cfg.service(delete);
}
