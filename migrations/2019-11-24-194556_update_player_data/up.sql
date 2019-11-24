-- Your SQL goes here

ALTER TABLE players_data ADD COLUMN last_updated TIMESTAMP NOT NULL DEFAULT current_timestamp;