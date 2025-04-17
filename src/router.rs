use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    errors::AppError,
    handlers::{
        channel_handler::{
            create_channel, get_channel, join_channel, leave_channel, list_channels,
        },
        user_handler::{login, register},
    },
    state::AppState,
};

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/api/v1/users/register", post(register))
        .route("/api/v1/users/login", post(login))
        .route("/api/v1/channels/join", post(join_channel))
        .route("/api/v1/channels/leave", post(leave_channel))
        .route("/api/v1/channels/{channel_id}", get(get_channel))
        .route("/api/v1/channels", post(create_channel).get(list_channels))
        .route("/api/v1/threads", post(create_thread))
        .route("/api/v1/messages", post(send_message))
        .with_state(state);

    Ok(api_router)
}

async fn index() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn create_thread() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn send_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
