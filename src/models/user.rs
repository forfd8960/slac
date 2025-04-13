use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::chrono};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub display_name: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
