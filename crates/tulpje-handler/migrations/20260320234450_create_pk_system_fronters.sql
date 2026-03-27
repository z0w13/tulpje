CREATE TABLE pk_system_fronters (
    system_uuid UUID PRIMARY KEY REFERENCES pk_systems(uuid),
    fronters JSONB NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
