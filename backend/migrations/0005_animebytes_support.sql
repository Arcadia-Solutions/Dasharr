INSERT INTO indexers (name, auth_data) VALUES
('AnimeBytes', '{"api_key": {"value": "", "explanation": "Get it from your profile''s settings > Account"}}');

ALTER TABLE user_profiles ADD uploaded_real BIGINT;
ALTER TABLE user_profiles ADD downloaded_real BIGINT;
ALTER TABLE user_profiles ADD seed_size BIGINT;
ALTER TABLE user_profiles ADD average_seed_time BIGINT;
