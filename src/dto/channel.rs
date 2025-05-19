use crate::models::channel::{Channel as ChanDao, ChannelMembers};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    pub id: i64,
    pub ch_name: String,
    pub ch_description: String,
    pub creator_id: i64,
    pub is_private: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChannelRequest {
    #[validate(length(min = 5, max = 50))]
    pub ch_name: String,
    #[validate(length(min = 8))]
    pub ch_desc: String,
    pub creator_id: i64,
    pub is_private: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateChannelResp {
    pub channel: Channel,
}

#[derive(Debug, Deserialize)]
pub struct ListChanReq {
    pub creator_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ListChanResp {
    pub channels: Vec<Channel>,
}

#[derive(Debug, Serialize)]
pub struct GetChanResp {
    pub channel: Option<Channel>,
}

#[derive(Debug, Deserialize)]
pub struct JoinChanReq {
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct LeaveChanReq {
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct JoinChanResp {
    pub chan_members: ChannelMembers,
}

#[derive(Debug, Serialize)]
pub struct LeaveChanResp {
    pub chan_members: Vec<ChannelMembers>,
}

#[derive(Debug, Serialize)]
pub struct ListChanMembersResp {
    pub chan_members_list: Vec<ChannelMembers>,
}

impl From<ChanDao> for Channel {
    fn from(ch: ChanDao) -> Self {
        Self {
            id: ch.id,
            ch_name: ch.ch_name,
            ch_description: ch.ch_description,
            creator_id: ch.creator_id,
            is_private: ch.is_private,
            is_archived: ch.is_archived,
            created_at: ch.created_at,
            updated_at: ch.updated_at,
        }
    }
}
