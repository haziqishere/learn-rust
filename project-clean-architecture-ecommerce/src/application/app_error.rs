use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid credentials error")]
    InvalidCredentials,

    #[error("Internal server: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;
