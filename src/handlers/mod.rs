use sqlx::{Pool, Postgres};

use crate::{
    dto::{
        SimpleUser,
        channel::ListChanMembersResp,
        message::{Message, SendMessageReq},
    },
    errors::AppError,
    models::{channel::ChanRepository, message::MessageStore, user::UserRepository},
    service::{channel::ChannelService, message::MsgService},
};

pub mod channel_handler;
pub mod message_handler;
pub mod user_handler;
pub mod websocket;

pub async fn list_channel_memebers(
    pool: &Pool<Postgres>,
    channel_id: i64,
) -> Result<ListChanMembersResp, AppError> {
    println!("list channel {} members", channel_id);

    let user_repo = UserRepository::new(pool);
    let chan_repo = ChanRepository::new(pool);
    let chan_service = ChannelService::new(&chan_repo, &user_repo);

    let resp = chan_service.list_channel_members(channel_id).await?;
    println!("list members response: {:?}", resp);
    Ok(resp)
}

pub async fn send_message_to_channel(
    pool: &Pool<Postgres>,
    channel_id: i64,
    req: &SendMessageReq,
) -> Result<Message, AppError> {
    println!("send messages to {}", channel_id);
    println!("send message req: {:?}", req);

    let user_repo = UserRepository::new(pool);
    let chan_repo = ChanRepository::new(pool);
    let msg_store = MessageStore::new(pool);
    let msg_service = MsgService::new(&chan_repo, &user_repo, &msg_store);

    let msg_dao = msg_service.send_message(channel_id, req).await?;
    let msg: Message = msg_dao.into();
    println!("msg: {:?}", msg);
    Ok(msg)
}

pub async fn list_simple_users(
    pool: &Pool<Postgres>,
    ids: Vec<i64>,
) -> Result<Vec<SimpleUser>, AppError> {
    let user_repo = UserRepository::new(pool);
    let result = user_repo.get_user_by_ids(ids).await?;

    let simple_users = result
        .iter()
        .map(|u| SimpleUser {
            id: u.id,
            display_name: u.display_name.clone(),
            avatar_url: u.avatar_url.clone(),
        })
        .collect();

    println!("list members response: {:?}", simple_users);
    Ok(simple_users)
}
