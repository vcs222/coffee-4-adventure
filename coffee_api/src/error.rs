use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),

    #[error("Not found: {message}")]
    NotFound { message: String },

    // #[error("Bad request: {message}")]
    // BadRequest { message: String },

    #[error("Internal server error: {message}")]
    Internal { message: String },

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(err) => {
                eprintln!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", err),
                )
            }
            ApiError::NotFound { message } => (StatusCode::NOT_FOUND, message),
            // ApiError::BadRequest { message } => (StatusCode::BAD_REQUEST, message),
            ApiError::Internal { message } => {
                eprintln!("Internal error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            ApiError::EnvVar(err) => {
                eprintln!("Environment variable error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Configuration error: {}", err),
                )
            }
            ApiError::Io(err) => {
                eprintln!("IO error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("IO error: {}", err),
                )
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
