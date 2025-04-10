-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(128) NOT NULL UNIQUE,
    password VARCHAR(64) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS channels (
    id BIGSERIAL PRIMARY KEY,
    creator_id BIGINT NOT NULL,
    name VARCHAR(128) NOT NULL UNIQUE,
    description VARCHAR(256) NOT NULL,
    is_public BOOLEAN NOT NULL,
    user_list BIGINT[] NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('ACTIVE', 'ARCHIVED')),
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_channels_creator_id ON channels(creator_id);

CREATE TABLE IF NOT EXISTS threads(
    id BIGSERIAL PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    from_user BIGINT NOT NULL,
    msg_list BIGINT[] NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_threads_channel_id ON threads(channel_id);

CREATE INDEX idx_threads_from_user ON threads(from_user);

CREATE TABLE IF NOT EXISTS messages (
    id BIGSERIAL PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    thread_id BIGINT NOT NULL,
    msg_type SMALLINT NOT NULL CHECK (msg_type IN (0, 1)),
    from_user BIGINT NOT NULL,
    to_user BIGINT NOT NULL,
    content JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_messages_channel_id ON messages(channel_id);

CREATE INDEX idx_messages_thread_id ON messages(thread_id);

CREATE INDEX idx_messages_from_user ON messages(from_user);

CREATE INDEX idx_messages_to_user ON messages(to_user);
