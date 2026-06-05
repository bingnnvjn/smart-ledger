use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid param: {0}")]
    InvalidParam(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("AI error: {0}")]
    AiError(String),
}

pub type AppResult<T> = Result<T, AppError>;
