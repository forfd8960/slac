use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::chrono};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
