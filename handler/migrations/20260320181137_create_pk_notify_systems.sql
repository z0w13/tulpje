CREATE TABLE pk_notify_systems (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    system_uuid UUID NOT NULL REFERENCES pk_systems(uuid),

    UNIQUE(guild_id, system_uuid)
);
