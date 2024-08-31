CREATE TABLE stocks (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    symbol TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    similar_stocks TEXT [] DEFAULT '{}'
);

CREATE TABLE processed_stocks (
    id SERIAL PRIMARY KEY,
    stock_id INT REFERENCES stocks(id),
    mp4_url TEXT NOT NULL,
    thumbnail_url TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);