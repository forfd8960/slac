use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    dto::message::ListMessagesReq,
    errors::AppError,
    models::{channel::ChanRepository, message::MessageStore, user::UserRepository},
    service::message::MsgService,
    state::AppState,
};

#[debug_handler]
pub async fn list_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<i64>,
    Json(req): Json<ListMessagesReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("list {} messages", channel_id);
    println!("list messages req: {:?}", req);

    let user_repo = UserRepository::new(&state.pool);
    let chan_repo = ChanRepository::new(&state.pool);
    let msg_store = MessageStore::new(&state.pool);
    let msg_service = MsgService::new(&chan_repo, &user_repo, &msg_store);

    let resp = msg_service.list_messages(channel_id, &req).await?;
    println!("list msg resp: {:?}", resp);
    Ok(Json(resp))
}

pub async fn send_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World")
}

pub async fn update_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World")
}
