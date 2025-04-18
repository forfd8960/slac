use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::errors::AppError;

const ROLE_MEMBER: &'static str = "member";
const ROLE_ADMIN: &'static str = "admin";

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

pub struct CreateChannel {
    pub ch_name: String,
    pub ch_description: String,
    pub creator_id: i64,
    pub is_private: bool,
    pub is_archived: bool,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ChannelMembers {
    pub id: i64,
    pub user_id: i64,
    pub channel_id: i64,
    pub member_role: String, // member, admin
    pub joined_at: chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub struct ChanRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ChanRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, channel: &CreateChannel) -> Result<Channel, AppError> {
        let created_channel = sqlx::query_as(
            r#"
            INSERT INTO channels 
            (ch_name, ch_description, creator_id, is_private, is_archived)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(&channel.ch_name)
        .bind(&channel.ch_description)
        .bind(&channel.creator_id)
        .bind(&channel.is_private)
        .bind(&channel.is_archived)
        .fetch_one(self.pool)
        .await?;

        Ok(created_channel)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Channel>, AppError> {
        let channel = sqlx::query_as(
            r#"
            SELECT * FROM channels WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(channel)
    }

    // Update
    pub async fn update(&self, channel: &Channel) -> Result<Option<Channel>, AppError> {
        let updated_channel = sqlx::query_as(
            r#"
            UPDATE channels 
            SET 
                ch_name = $1,
                ch_description = $2,
                is_private = $3,
                is_archived = $4,
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(&channel.ch_name)
        .bind(&channel.ch_description)
        .bind(&channel.is_private)
        .bind(&channel.is_archived)
        .bind(channel.id)
        .fetch_optional(self.pool)
        .await?;

        Ok(updated_channel)
    }

    pub async fn list_all(&self, creator_id: i64) -> Result<Vec<Channel>, AppError> {
        let channels = sqlx::query_as(
            r#"
            SELECT * FROM channels WHERE creator_id=$1
            "#,
        )
        .bind(creator_id)
        .fetch_all(self.pool)
        .await?;

        Ok(channels)
    }

    pub async fn list_channel_members(
        &self,
        channel_id: i64,
    ) -> Result<Vec<ChannelMembers>, AppError> {
        let ch_members = sqlx::query_as(
            r#"
            SELECT * FROM channel_members WHERE channel_id=$1
            "#,
        )
        .bind(channel_id)
        .fetch_all(self.pool)
        .await?;

        Ok(ch_members)
    }

    pub async fn add_channel_member(
        &self,
        channel_id: i64,
        user_id: i64,
    ) -> Result<ChannelMembers, AppError> {
        let chan_member: ChannelMembers = sqlx::query_as(
            r#"
            INSERT INTO channel_members 
            (user_id, channel_id, member_role)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(channel_id)
        .bind(ROLE_MEMBER)
        .fetch_one(self.pool)
        .await?;

        Ok(chan_member)
    }
}
