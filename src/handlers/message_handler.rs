use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    dto::message::{ListMessagesReq, ListMessagesResp, Message, SendMessageReq},
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

    let messages = msg_service.list_messages(channel_id, &req).await?;
    let messages1: Vec<Message> = messages.into_iter().map(|v| v.into()).collect();
    let resp = ListMessagesResp {
        msgs: messages1,
        has_more: false,
    };
    println!("list msg resp: {:?}", resp);
    Ok(Json(resp))
}

pub async fn send_message_to_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<i64>,
    Json(req): Json<SendMessageReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("send messages to {}", channel_id);
    println!("send message req: {:?}", req);

    let user_repo = UserRepository::new(&state.pool);
    let chan_repo = ChanRepository::new(&state.pool);
    let msg_store = MessageStore::new(&state.pool);
    let msg_service = MsgService::new(&chan_repo, &user_repo, &msg_store);

    let msg_dao = msg_service.send_message(channel_id, &req).await?;
    let resp: Message = msg_dao.into();
    println!("send msg resp: {:?}", resp);
    Ok(Json(resp))
}

pub async fn get_message(
    State(state): State<AppState>,
    Path(message_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    println!("get message {}", message_id);

    let user_repo = UserRepository::new(&state.pool);
    let chan_repo = ChanRepository::new(&state.pool);
    let msg_store = MessageStore::new(&state.pool);
    let msg_service = MsgService::new(&chan_repo, &user_repo, &msg_store);

    let msg_dao = msg_service.get_message(message_id).await?;
    if msg_dao.is_none() {
        return Err(AppError::NotFound("message not found".to_string()));
    }

    let resp: Message = msg_dao.unwrap().into();
    println!("get msg resp: {:?}", resp);
    Ok(Json(resp))
}

pub async fn update_message() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World")
}
