-- Your SQL goes here

ALTER TABLE users ADD COLUMN player_data_id UUID REFERENCES players_data(id) ON
DELETE CASCADE;