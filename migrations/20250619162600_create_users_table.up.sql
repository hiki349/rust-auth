-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) DEFAULT '' NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
