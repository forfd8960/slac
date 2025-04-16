use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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

        (status_code, format!("{:?}", self)).into_response()
    }
}
