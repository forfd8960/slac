use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Channel {
    pub id: i64,
    pub ch_name: String,
    pub ch_description: String,
    pub creator_id: i64,
    pub is_private: bool,  // public or private channel
    pub is_archived: bool, // ACTIVE, ARCHIVED
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ChannelMembers {
    pub id: i64,
    pub user_id: i64,
    pub channel_id: i64,
    pub member_role: String, // member, admin
    pub joined_at: chrono::DateTime<Utc>,
}
