use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

type Result<T> = std::result::Result<T, AppError>;