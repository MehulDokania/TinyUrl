-- Your SQL goes here
CREATE TABLE url_map (
    id SERIAL PRIMARY KEY,
    original_url TEXT NOT NULL,
    tiny_url TEXT NOT NULL UNIQUE,
    fetch_count INTEGER NOT NULL DEFAULT 0
);