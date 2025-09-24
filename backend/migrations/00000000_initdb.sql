CREATE TABLE indexers (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    auth_data JSONB NOT NULL DEFAULT '[]'
);
