INSERT INTO indexers (name, auth_data) VALUES
('PhoenixProject', '{"api_key": {"value": "", "explanation": "Get it from your profile''s settings, in \"Access Settings\""},"user_id": {"value": "", "explanation": null}}');

ALTER TABLE user_profiles ALTER COLUMN rank_overall TYPE REAL;
