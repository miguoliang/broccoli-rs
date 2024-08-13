use diesel::prelude::*;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::pattern::{EDGE_LABEL_LIKE, USERNAME_LIKE};
use crate::schema;

#[derive(Debug, Deserialize, Validate, Insertable)]
#[diesel(table_name = schema::vertex)]
pub struct NewVertex {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub created_by: String,
}

#[derive(Debug, Deserialize, Validate, Insertable)]
#[diesel(table_name = schema::edge)]
#[validate(schema(function = "vertices_not_same"))]
pub struct NewEdge {
    #[validate(range(min = 1))]
    pub from_vertex_id: i32,
    #[serde(skip_deserializing)]
    pub from_vertex_type: String,
    #[validate(range(min = 1))]
    pub to_vertex_id: i32,
    #[serde(skip_deserializing)]
    pub to_vertex_type: String,
    #[validate(regex(path = *EDGE_LABEL_LIKE))]
    pub label: String,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub created_by: String,
}

fn vertices_not_same(new_edge: &NewEdge) -> Result<(), ValidationError> {
    if new_edge.from_vertex_id != new_edge.to_vertex_id {
        Ok(())
    } else {
        Err(ValidationError::new("matched"))
    }
}
