use axum::{
    Router,
    response::IntoResponse,
    routing::{any, delete, get, post, put},
};

use crate::{
    errors::AppError,
    handlers::{
        channel_handler::{
            create_channel, get_channel, join_channel, leave_channel, list_channel_memebers,
            list_channels,
        },
        message_handler::{get_message, list_messages, send_message_to_channel, update_message},
        user_handler::{get_user, login, register},
        websocket::message_loop,
    },
    state::AppState,
};

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/{user_id}/websocket", any(message_loop))
        .route("/api/v1/users/register", post(register))
        .route("/api/v1/users/login", post(login))
        .route("/api/v1/users/{user_id}", get(get_user))
        .route("/api/v1/channels/{channel_id}/join", post(join_channel))
        .route("/api/v1/channels/{channel_id}/leave", delete(leave_channel))
        .route("/api/v1/channels/{channel_id}", get(get_channel))
        .route(
            "/api/v1/channels/{channel_id}/messages",
            get(list_messages).post(send_message_to_channel),
        )
        .route(
            "/api/v1/channels/{channel_id}/members",
            get(list_channel_memebers),
        )
        .route("/api/v1/channels", post(create_channel).get(list_channels))
        .route("/api/v1/messages", put(update_message))
        .route("/api/v1/messages/{message_id}", get(get_message))
        .with_state(state);

    Ok(api_router)
}

async fn index() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
