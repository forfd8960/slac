use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Thread {
    pub id: i64,
    pub channel_id: i64,
    pub from_user: i64,
    pub msg_list: Vec<i64>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub channel_id: i64,
    pub thread_id: i64,
    // 0: direct message, 1: channel message
    pub msg_type: u8,
    pub from_user: i64,
    pub to_user: i64,
    pub content: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
