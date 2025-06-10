use serde_json::json;

use crate::{
    dto::message::{ListMessagesReq, SendMessageReq},
    errors::AppError,
    models::{
        channel::ChanRepository,
        message::{CreateMessage, Message, MessageStore},
        user::UserRepository,
    },
};

pub struct MsgService<'a> {
    chan_store: &'a ChanRepository<'a>,
    user_store: &'a UserRepository<'a>,
    msg_store: &'a MessageStore<'a>,
}

impl<'a> MsgService<'a> {
    pub fn new(
        chan_store: &'a ChanRepository,
        user_store: &'a UserRepository,
        msg_store: &'a MessageStore,
    ) -> Self {
        Self {
            chan_store,
            user_store,
            msg_store,
        }
    }

    pub async fn get_message(&self, msg_id: i64) -> Result<Option<Message>, AppError> {
        let message = self.msg_store.get_by_id(msg_id).await?;
        Ok(message)
    }

    pub async fn list_messages(
        &self,
        chan_id: i64,
        list_req: &ListMessagesReq,
    ) -> Result<Vec<Message>, AppError> {
        let messages = self
            .msg_store
            .list_by_channel(chan_id, list_req.limit, list_req.offset)
            .await?;

        Ok(messages)
    }

    pub async fn send_message(
        &self,
        chan_id: i64,
        send_req: &SendMessageReq,
    ) -> Result<Message, AppError> {
        let chan = self.chan_store.get_by_id(chan_id).await?;
        if chan.is_none() {
            return Err(AppError::NotFound("channel not found".to_string()));
        }

        let media_meta = json!(send_req.media_metadata);
        let msg = self
            .msg_store
            .create(&CreateMessage {
                sender_id: send_req.sender_id,
                channel_id: chan_id,
                parent_msg_id: send_req.parent_msg_id,
                content_type: send_req.content_type.clone().into(),
                text_content: send_req.text_content.clone(),
                media_url: send_req.media_url.clone(),
                media_metadata: media_meta,
            })
            .await?;

        //todo: send to msg queue, and broadcast to all users.
        Ok(msg)
    }
}
