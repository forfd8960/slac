use sqlx::{Pool, Postgres};

use crate::{
    dto::channel::ListChanMembersResp,
    errors::AppError,
    models::{channel::ChanRepository, user::UserRepository},
    service::channel::ChannelService,
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
