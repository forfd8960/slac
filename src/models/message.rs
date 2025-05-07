use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::errors::AppError;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub channel_id: i64,
    pub sender_id: Option<i64>,
    pub parent_msg_id: Option<i64>,
    pub content_type: String,
    pub text_content: String,
    pub media_url: Option<String>,
    pub media_metadata: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CreateMessage {
    pub channel_id: i64,
    pub sender_id: Option<i64>,
    pub parent_msg_id: Option<i64>,
    pub content_type: String,
    pub text_content: String,
    pub media_url: Option<String>,
    pub media_metadata: Option<String>,
}

pub struct MessageStore<'a> {
    pub pool: &'a PgPool,
}

impl<'a> MessageStore<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_message: &CreateMessage) -> Result<Message, AppError> {
        let message = sqlx::query_as(
            r#"
            INSERT INTO messages (
                channel_id, 
                sender_id, 
                parent_msg_id, 
                content_type, 
                text_content, 
                media_url, 
                media_metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(new_message.channel_id)
        .bind(new_message.sender_id)
        .bind(new_message.parent_msg_id)
        .bind(&new_message.content_type)
        .bind(&new_message.text_content)
        .bind(&new_message.media_url)
        .bind(&new_message.media_metadata)
        .fetch_one(self.pool)
        .await?;

        Ok(message)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Message>, AppError> {
        let message = sqlx::query_as(
            r#"
            SELECT * FROM messages WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(message)
    }

    pub async fn update(&self, message: &Message) -> Result<Option<Message>, AppError> {
        let updated_message = sqlx::query_as(
            r#"
            UPDATE messages
            SET 
                text_content = $1,
                media_url = $2,
                media_metadata = $3,
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(&message.text_content)
        .bind(&message.media_url)
        .bind(&message.media_metadata)
        .bind(&message.id)
        .fetch_optional(self.pool)
        .await?;

        Ok(updated_message)
    }

    pub async fn list_by_channel(
        &self,
        channel_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, AppError> {
        let messages = sqlx::query_as(
            r#"
            SELECT * FROM messages 
            WHERE channel_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(channel_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        Ok(messages)
    }

    pub async fn get_replies(&self, parent_msg_id: i64) -> Result<Vec<Message>, AppError> {
        let replies = sqlx::query_as(
            r#"
            SELECT * FROM messages 
            WHERE parent_msg_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(parent_msg_id)
        .fetch_all(self.pool)
        .await?;

        Ok(replies)
    }
}
