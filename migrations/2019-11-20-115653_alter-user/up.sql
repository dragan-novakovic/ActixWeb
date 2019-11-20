-- Your SQL goes here

ALTER TABLE players_data
DROP COLUMN user_id;


ALTER TABLE players_data ADD COLUMN id UUID PRIMARY KEY DEFAULT uuid_generate_v4();