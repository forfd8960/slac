use serde::{Deserialize, Serialize};

use crate::models::message::Message;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Image,
    Video,
    File,
    System,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SendMessageReq {
    pub chan_id: i64,
    pub sender_id: Option<i64>,
    pub parent_msg_id: Option<i64>,
    pub content_type: ContentType,
    pub text_content: String,
    pub media_url: Option<String>,
    pub media_metadata: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UpdateMessageReq {
    pub id: i64,
    pub chan_id: i64,
    pub sender_id: Option<i64>,
    pub parent_msg_id: Option<i64>,
    pub content_type: ContentType,
    pub text_content: String,
    pub media_url: Option<String>,
    pub media_metadata: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SendMessageResp {
    pub msg: Message,
}

#[derive(Debug, Serialize)]
pub struct ListMessagesResp {
    pub msgs: Vec<Message>,
}
