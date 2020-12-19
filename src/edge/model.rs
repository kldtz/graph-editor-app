use crate::api_error::GraphError;
use crate::db;
use crate::node::Node;
use crate::schema::edge;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "edge"]
pub struct EdgeInit {
    pub edge_label: String,
    pub source_id: i32,
    pub target_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "edge"]
pub struct EdgePatch {
    pub edge_label: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(parent = "Node", foreign_key = "source_id")]
#[table_name = "edge"]
pub struct Edge {
    pub id: i32,
    pub edge_label: String,
    pub source_id: i32,
    pub target_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Edge {
    pub fn find(id: i32) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let edge = edge::table.filter(edge::id.eq(id)).first(&conn)?;

        Ok(edge)
    }

    pub fn create(edge: EdgeInit) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let edge = diesel::insert_into(edge::table)
            .values(edge)
            .get_result(&conn)?;

        Ok(edge)
    }

    pub fn patch(id: i32, edge: EdgePatch) -> Result<Self, GraphError> {
        let conn = db::connection()?;

        let edge = diesel::update(edge::table)
            .filter(edge::id.eq(id))
            .set(edge)
            .get_result(&conn)?;

        Ok(edge)
    }

    pub fn delete(id: i32) -> Result<usize, GraphError> {
        let conn = db::connection()?;

        let res = diesel::delete(edge::table.filter(edge::id.eq(id))).execute(&conn)?;

        Ok(res)
    }
}
