-- Your SQL goes here

ALTER TABLE players_data
ALTER COLUMN player_inventory_id
SET NOT NULL;


ALTER TABLE players_data
ALTER COLUMN player_stats_id
SET NOT NULL;

