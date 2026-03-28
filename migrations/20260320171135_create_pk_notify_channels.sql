CREATE TABLE pk_notify_channels (
    guild_id BIGINT PRIMARY KEY,
    channel_id BIGINT NOT NULL,

    UNIQUE(guild_id, channel_id)
);
