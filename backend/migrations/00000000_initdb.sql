CREATE TABLE indexers (
    id SERIAL PRIMARY KEY,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    name TEXT UNIQUE NOT NULL,
    auth_data JSONB NOT NULL DEFAULT '[]'
);

INSERT INTO indexers (name, auth_data) VALUES
('Redacted', '{"api_key": {"value": "", "explanation": "Get it from your profile''s settings, in \"Access Settings\""},"user_id": {"value": "", "explanation": null}}'),
('GazelleGames', '{"api_key": {"value": "", "explanation": "Get it from your profile''s settings"},"user_id": {"value": "", "explanation": null}}'),
('Orpheus', '{"api_key": {"value": "", "explanation": "Get it from your profile''s settings, in \"Access Settings\""},"user_id": {"value": "", "explanation": null}}');

CREATE TABLE user_profiles (
    scraped_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    indexer_id INT NOT NULL REFERENCES indexers(id),
    avatar TEXT NOT NULL,
    -- last_access TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    uploaded BIGINT NOT NULL,
    downloaded BIGINT NOT NULL,
    ratio REAL NOT NULL,
    required_ratio REAL,
    class VARCHAR(50) NOT NULL,
    rank_uploaded INT,
    rank_downloaded INT,
    rank_uploads INT,
    rank_requests INT,
    rank_bounty INT,
    rank_posts INT,
    rank_artists INT,
    rank_overall INT,
    -- paranoia INT,
    paranoia_text TEXT,
    donor BOOLEAN,
    warned BOOLEAN,
    posts INT,
    torrent_comments INT,
    collages_started INT,
    collages_contrib INT,
    requests_filled INT,
    requests_voted INT,
    uploaded_torrents INT,
    groups INT,
    seeding INT,
    leeching INT,
    snatched INT,
    invited INT,
    bonus_points BIGINT
);
