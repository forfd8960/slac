-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(128) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS channels (
    id BIGSERIAL PRIMARY KEY,
    ch_name VARCHAR(128) NOT NULL UNIQUE,
    ch_description TEXT NOT NULL DEFAULT '',
    creator_id BIGINT NOT NULL,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_channels_creator_id ON channels(creator_id);

CREATE TABLE IF NOT EXISTS channel_members (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    -- 'member','admin',
    member_role VARCHAR(20) NOT NULL DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_user_channel UNIQUE (user_id, channel_id)
);

-- Find channels a user is in
CREATE INDEX idx_channel_members_user_id ON channel_members(user_id);
-- Find members of a channel
CREATE INDEX idx_channel_members_channel_id ON channel_members(channel_id);

CREATE TYPE message_content_type AS ENUM ('text', 'image', 'video', 'file', 'system');

CREATE TABLE IF NOT EXISTS messages (
    id BIGSERIAL PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    sender_id BIGINT, -- Can be NULL if sender user is deleted or for system messages
    parent_msg_id BIGINT, -- For threading: references the 'id' of the message this is replying to
    content_type message_content_type NOT NULL DEFAULT 'text',
    text_content TEXT, -- Content for 'text' type messages
    media_url VARCHAR(2048), -- URL for 'image', 'video', 'file' types (points to Object Storage)
    media_metadata JSONB, -- Optional: Store metadata like filename, size, dimensions, duration
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Fetch messages for a channel, ordered by time (most common query)
CREATE INDEX idx_messages_channel_created ON messages(channel_id, created_at DESC);

CREATE INDEX idx_messages_sender_id ON messages(sender_id, created_at DESC);

-- Fetch replies to a specific message (thread view), ordered by time
CREATE INDEX idx_messages_parent_created ON messages(parent_msg_id, created_at ASC);