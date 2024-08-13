use diesel::prelude::*;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::pattern::{EDGE_LABEL_LIKE, USERNAME_LIKE};
use crate::schema;

#[derive(Debug, Deserialize, Validate)]
pub struct NewVertex {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub created_by: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::vertex)]
pub struct InsertableNewVertex {
    pub name: String,
    pub type_: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "vertices_not_same"))]
pub struct NewEdge {
    #[validate(range(min = 1))]
    pub from_vertex_id: i32,
    #[validate(range(min = 1))]
    pub to_vertex_id: i32,
    #[validate(regex(path = *EDGE_LABEL_LIKE))]
    pub label: String,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub created_by: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::edge)]
pub struct InsertableNewEdge {
    pub from_vertex_id: i32,
    pub from_vertex_type: String,
    pub to_vertex_id: i32,
    pub to_vertex_type: String,
    pub label: String,
    pub created_by: String,
    pub updated_by: String,
}

fn vertices_not_same(new_edge: &NewEdge) -> Result<(), ValidationError> {
    if new_edge.from_vertex_id != new_edge.to_vertex_id {
        Ok(())
    } else {
        Err(ValidationError::new("matched"))
    }
}
