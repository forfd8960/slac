use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("{0} not found")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, format!("{:?}", self)).into_response()
    }
}
