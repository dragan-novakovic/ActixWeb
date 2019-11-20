-- Your SQL goes here

ALTER TABLE users
ALTER COLUMN player_data_id
SET NOT NULL;


ALTER TABLE players_data
ALTER COLUMN factories_id
SET NOT NULL;


ALTER TABLE player_factories
ALTER COLUMN user_id
SET NOT NULL;


ALTER TABLE player_factories
ALTER COLUMN factory_id
SET NOT NULL;