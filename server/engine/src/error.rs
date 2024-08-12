use diesel::result::Error as DieselError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("Diesel error: {0}")]
    Database(#[from] DieselError),
    #[error("Other error: {0}")]
    Other(#[from] std::io::Error),
}
