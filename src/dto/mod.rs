use serde::{Deserialize, Serialize};

pub mod channel;
pub mod message;
pub mod user;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct SimpleUser {
    pub id: i64,
    pub avatar_url: String,
    pub display_name: String,
}
