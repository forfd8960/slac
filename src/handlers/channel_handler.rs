use axum::response::IntoResponse;

use crate::errors::AppError;

pub async fn list_channels() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn get_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn join_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn leave_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn create_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
