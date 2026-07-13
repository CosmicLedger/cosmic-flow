use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

pub type Result<T> = std::result::Result<T, IndexerError>;

#[derive(Debug)]
pub enum IndexerError {
    Database(sqlx::Error),
    Http(reqwest::Error),
    Config(String),
    NotFound(String),
    Validation(String),
    Internal(String),
}

impl fmt::Display for IndexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexerError::Database(e) => write!(f, "Database error: {}", e),
            IndexerError::Http(e) => write!(f, "HTTP error: {}", e),
            IndexerError::Config(msg) => write!(f, "Configuration error: {}", msg),
            IndexerError::NotFound(msg) => write!(f, "Not found: {}", msg),
            IndexerError::Validation(msg) => write!(f, "Validation error: {}", msg),
            IndexerError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for IndexerError {}

impl ResponseError for IndexerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            IndexerError::NotFound(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "not_found",
                    "message": msg
                }))
            }
            IndexerError::Validation(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "validation_error",
                    "message": msg
                }))
            }
            IndexerError::Database(_) | IndexerError::Internal(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_error",
                    "message": "An internal error occurred"
                }))
            }
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "internal_error",
                "message": "An unexpected error occurred"
            })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            IndexerError::NotFound(_) => StatusCode::NOT_FOUND,
            IndexerError::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for IndexerError {
    fn from(err: sqlx::Error) -> Self {
        IndexerError::Database(err)
    }
}

impl From<reqwest::Error> for IndexerError {
    fn from(err: reqwest::Error) -> Self {
        IndexerError::Http(err)
    }
}

impl From<std::io::Error> for IndexerError {
    fn from(err: std::io::Error) -> Self {
        IndexerError::Internal(err.to_string())
    }
}
