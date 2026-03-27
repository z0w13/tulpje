CREATE TABLE pk_systems (
    uuid UUID PRIMARY KEY,
    id VARCHAR(6) UNIQUE NOT NULL,
    name TEXT
);
