-- Your SQL goes here

ALTER TABLE factories
ALTER COLUMN id
SET DEFAULT uuid_generate_v4();