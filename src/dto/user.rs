use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub avatar_url: String,
    pub display_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 6, max = 20))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(url)]
    pub avatar: String,

    #[validate(length(min = 1, max = 20))]
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user: User,
}
