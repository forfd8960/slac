use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{errors::AppError, state::AppState};

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/chats/v1/users/register", get(register))
        .route("/chats/v1/users/login", post(login))
        .route("/chats/v1/convertions/join", post(join_convs))
        .route("/chats/v1/convertions/leave", post(leave_convs))
        .route("/chats/v1/convertions", post(create_convs))
        .route("/chats/v1/messages", post(send_message))
        .with_state(state);

    Ok(api_router)
}

async fn index() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn register() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn login() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn join_convs() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn leave_convs() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn create_convs() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn send_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
