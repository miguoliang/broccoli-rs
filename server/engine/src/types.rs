use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

const MAX_NAME_LENGTH: usize = 255;
const MAX_TYPE_LENGTH: usize = 255;
const MAX_USERNAME_LENGTH: usize = 255;

static NOT_BLANK: Lazy<Regex> = Lazy::new(|| Regex::new(r"\S+").unwrap());

pub static USERNAME_LIKE: Lazy<Regex> =
    Lazy::new(|| Regex::new(format!("^[a-zA-Z0-9]{{3,{MAX_USERNAME_LENGTH}}}$").as_str()).unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVertex {
    pub name: String,
    pub r#type: String,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub created_by: String,
}

#[derive(Debug, Serialize)]
pub struct Vertex {
    pub id: i32,
    pub name: String,
    pub r#type: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
