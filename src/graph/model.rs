use crate::api_error::GraphError;
use crate::db;
use crate::edge::Edge;
use crate::node::Node;
use crate::schema::graph;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Debug)]
#[table_name = "graph"]
pub struct GraphInit {
    pub graph_label: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Debug)]
#[table_name = "graph"]
pub struct GraphPatch {
    pub translate_x: Option<f64>,
    pub translate_y: Option<f64>,
    pub scale: Option<f64>,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Insertable)]
#[table_name = "graph"]
pub struct Graph {
    pub id: i32,
    pub graph_label: String,
    pub translate_x: f64,
    pub translate_y: f64,
    pub scale: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Graph {
    pub fn find_all() -> Result<Vec<Self>, GraphError> {
        let conn = db::connection()?;

        let graphs = graph::table.load::<Graph>(&conn)?;
        Ok(graphs)
    }

    pub fn find(id: i32) -> Result<Graph, GraphError> {
        let conn = db::connection()?;

        let graph = graph::table.filter(graph::id.eq(id)).first(&conn)?;
        Ok(graph)
    }

    pub fn create(graph: GraphInit) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let graph = diesel::insert_into(graph::table)
            .values(graph)
            .get_result(&conn)?;

        Ok(graph)
    }

    pub fn patch(id: i32, graph: GraphPatch) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let graph = diesel::update(graph::table)
            .filter(graph::id.eq(id))
            .set(graph)
            .get_result(&conn)?;

        Ok(graph)
    }

    pub fn delete(id: i32) -> Result<usize, GraphError> {
        let conn = db::connection()?;

        let res = diesel::delete(graph::table.filter(graph::id.eq(id))).execute(&conn)?;

        Ok(res)
    }
}

#[derive(Serialize)]
pub struct ExtendedGraph {
    pub graph: Graph,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl ExtendedGraph {
    fn new(graph: Graph, nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        ExtendedGraph {
            graph: graph,
            nodes: nodes,
            edges: edges,
        }
    }

    pub fn find(id: i32) -> Result<ExtendedGraph, GraphError> {
        let conn = db::connection()?;

        let graph: Graph = graph::table.filter(graph::id.eq(id)).first(&conn)?;
        let nodes = Node::belonging_to(&graph).load::<Node>(&conn)?;
        let edges = Edge::belonging_to(&nodes).load::<Edge>(&conn)?;
        let extended_graph = ExtendedGraph::new(graph, nodes, edges);
        Ok(extended_graph)
    }
}
