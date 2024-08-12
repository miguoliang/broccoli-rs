use crate::constant::{MAX_NAME_LENGTH, MAX_TYPE_LENGTH, MAX_USERNAME_LENGTH};
use once_cell::sync::Lazy;
use regex::Regex;

pub static NOT_BLANK: Lazy<Regex> = Lazy::new(|| Regex::new(r"\S+").unwrap());

pub static NAME_LIKE: Lazy<Regex> =
    Lazy::new(|| Regex::new(format!("^[a-zA-Z0-9]{{3,{MAX_NAME_LENGTH}}}$").as_str()).unwrap());

pub static TYPE_LIKE: Lazy<Regex> =
    Lazy::new(|| Regex::new(format!("^[a-zA-Z0-9]{{3,{MAX_TYPE_LENGTH}}}$").as_str()).unwrap());

pub static USERNAME_LIKE: Lazy<Regex> =
    Lazy::new(|| Regex::new(format!("^[a-zA-Z0-9]{{3,{MAX_USERNAME_LENGTH}}}$").as_str()).unwrap());
