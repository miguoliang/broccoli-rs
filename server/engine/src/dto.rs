use diesel::prelude::*;
use serde::Deserialize;
use validator::Validate;

use crate::pattern::USERNAME_LIKE;
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

#[derive(Debug, Deserialize, Validate)]
pub struct NewEdge {
    pub from_vertex_id: i32,
    pub to_vertex_id: i32,
    pub label: String,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub created_by: String,
}
