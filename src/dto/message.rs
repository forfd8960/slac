use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::message::Message as MessageDao;
use crate::models::message::MessageContentType as MessageCTDao;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MediaMetadata {
    width: u32,
    height: u32,
    format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageContentType {
    Text,
    Image,
    Video,
    File,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: i64,
    pub channel_id: i64,
    pub sender_id: Option<i64>,
    pub parent_msg_id: Option<i64>,
    pub content_type: MessageContentType,
    pub text_content: String,
    pub media_url: Option<String>,
    pub media_metadata: serde_json::Value,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SendMessageReq {
    pub sender_id: Option<i64>,
    pub parent_msg_id: Option<i64>,
    pub content_type: MessageContentType,
    pub text_content: String,
    pub media_url: Option<String>,
    pub media_metadata: Option<MediaMetadata>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UpdateMessageReq {
    pub id: i64,
    pub chan_id: i64,
    pub sender_id: Option<i64>,
    pub parent_msg_id: Option<i64>,
    pub content_type: MessageContentType,
    pub text_content: String,
    pub media_url: Option<String>,
    pub media_metadata: Option<MediaMetadata>,
}

#[derive(Debug, Serialize)]
pub struct SendMessageResp {
    pub msg: Message,
}

#[derive(Debug, Deserialize)]
pub struct ListMessagesReq {
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct ListMessagesResp {
    pub msgs: Vec<Message>,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageInSocket {
    pub channel_id: i64,
    pub msgs: Vec<SendMessageReq>,
}

impl From<MessageDao> for Message {
    fn from(msg: MessageDao) -> Self {
        Self {
            id: msg.id,
            channel_id: msg.channel_id,
            sender_id: msg.sender_id,
            parent_msg_id: msg.parent_msg_id,
            content_type: msg.content_type.into(),
            text_content: msg.text_content,
            media_url: msg.media_url,
            media_metadata: msg.media_metadata,
            created_at: msg.created_at,
            updated_at: msg.updated_at,
        }
    }
}

impl From<Message> for MessageDao {
    fn from(msg: Message) -> Self {
        Self {
            id: msg.id,
            channel_id: msg.channel_id,
            sender_id: msg.sender_id,
            parent_msg_id: msg.parent_msg_id,
            content_type: msg.content_type.into(),
            text_content: msg.text_content,
            media_url: msg.media_url,
            media_metadata: msg.media_metadata,
            created_at: msg.created_at,
            updated_at: msg.updated_at,
        }
    }
}

impl From<MessageCTDao> for MessageContentType {
    fn from(msg_ct: MessageCTDao) -> Self {
        match msg_ct {
            MessageCTDao::Text => MessageContentType::Text,
            MessageCTDao::Image => MessageContentType::Image,
            MessageCTDao::Video => MessageContentType::Video,
            MessageCTDao::File => MessageContentType::File,
            MessageCTDao::System => MessageContentType::System,
        }
    }
}

impl From<MessageContentType> for MessageCTDao {
    fn from(msg_ct: MessageContentType) -> Self {
        match msg_ct {
            MessageContentType::Text => MessageCTDao::Text,
            MessageContentType::Image => MessageCTDao::Image,
            MessageContentType::Video => MessageCTDao::Video,
            MessageContentType::File => MessageCTDao::File,
            MessageContentType::System => MessageCTDao::System,
        }
    }
}
