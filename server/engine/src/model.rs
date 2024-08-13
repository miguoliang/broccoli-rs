use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Selectable;
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::vertex)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vertex {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::edge)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Edge {
    pub id: i32,
    pub from_vertex_id: i32,
    pub from_vertex_type: String,
    pub to_vertex_id: i32,
    pub to_vertex_type: String,
    pub label: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}
