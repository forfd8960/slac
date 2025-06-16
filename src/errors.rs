use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("{0} not found")]
    NotFound(String),
    #[error("{0}")]
    InvalidArgument(String),

    #[error("{0}")]
    AlreadyExists(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("generate token failed: {0}")]
    GenerateTokenError(#[from] jwt_simple::Error),

    #[error("failed to generate hash: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::GenerateTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
        };

        // Create a JSON error response
        let error_response = ErrorResponse {
            error: format!("{:?}", self),
            code: status_code.as_u16(),
        };

        (status_code, Json(error_response)).into_response()
    }
}
