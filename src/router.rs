use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    dto::user::RegisterRequest, errors::AppError, handlers::user::UserService,
    models::user::UserRepository, state::AppState,
};

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/chats/v1/users/register", post(register))
        .route("/chats/v1/users/login", post(login))
        .route("/chats/v1/channels/join", post(join_channel))
        .route("/chats/v1/channels/leave", post(leave_channel))
        .route("/chats/v1/channels/{channel_id}", get(get_channel))
        .route(
            "/chats/v1/channels",
            post(create_channel).get(list_channels),
        )
        .route("/chats/v1/threads", post(create_thread))
        .route("/chats/v1/messages", post(send_message))
        .with_state(state);

    Ok(api_router)
}

async fn index() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("register user: {:?}", payload);

    let user_repo = UserRepository::new(&state.pool);
    let user_service = UserService::new(&user_repo);

    let resp = user_service.create_user(&payload).await?;
    println!("created user: {:?}", resp.user);
    Ok(Json(resp))
}

async fn login() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn list_channels() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn get_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn join_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn leave_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn create_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn create_thread() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn send_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
