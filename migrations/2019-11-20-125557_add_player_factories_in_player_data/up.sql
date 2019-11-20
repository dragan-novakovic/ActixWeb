-- Your SQL goes here

ALTER TABLE players_data ADD COLUMN factories_id UUID REFERENCES player_factories(id) ON
DELETE CASCADE;