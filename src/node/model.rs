use crate::api_error::GraphError;
use crate::db;
use crate::graph::Graph;
use crate::schema::node;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "node"]
pub struct NodeInit {
    pub node_label: String,
    pub graph_id: i32,
    pub x_coord: f64,
    pub y_coord: f64,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "node"]
pub struct NodePatch {
    pub node_label: Option<String>,
    pub x_coord: Option<f64>,
    pub y_coord: Option<f64>,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(parent = "Graph")]
#[table_name = "node"]
pub struct Node {
    pub id: i32,
    pub node_label: String,
    pub graph_id: i32,
    pub x_coord: f64,
    pub y_coord: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Node {
    pub fn find(id: i32) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let node = node::table.filter(node::id.eq(id)).first(&conn)?;

        Ok(node)
    }

    pub fn create(node: NodeInit) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let node = diesel::insert_into(node::table)
            .values(node)
            .get_result(&conn)?;

        Ok(node)
    }

    pub fn patch(id: i32, node: NodePatch) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let node = diesel::update(node::table)
            .filter(node::id.eq(id))
            .set(node)
            .get_result(&conn)?;

        Ok(node)
    }

    pub fn delete(id: i32) -> Result<usize, GraphError> {
        let conn = db::connection()?;

        let res = diesel::delete(node::table.filter(node::id.eq(id))).execute(&conn)?;

        Ok(res)
    }
}
