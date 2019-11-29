-- Your SQL goes here

ALTER TABLE players_data ADD COLUMN player_stats_id uuid REFERENCES player_stats(id),
                                                                    ADD COLUMN player_inventory_id uuid REFERENCES player_inventory(id);