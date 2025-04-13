use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub channel_id: i64,
    pub sender_id: i64,
    pub parent_msg_id: i64,
    pub content_type: String,
    pub text_content: String,
    pub media_url: String,
    pub media_metadata: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
