use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Channel {
    pub id: i64,
    pub creator_id: i64,
    pub name: String,
    pub description: String,
    pub is_public: bool, // public or private channel
    pub user_list: Vec<i64>,
    pub status: String, // ACTIVE, ARCHIVED
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
