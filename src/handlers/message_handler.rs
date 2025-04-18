use axum::response::IntoResponse;

use crate::errors::AppError;

pub async fn list_messages() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World")
}

pub async fn send_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World")
}

pub async fn update_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World")
}
