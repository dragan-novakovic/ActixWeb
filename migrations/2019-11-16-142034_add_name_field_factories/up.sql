-- Your SQL goes here

ALTER TABLE factories ADD COLUMN name VARCHAR(80);


ALTER TABLE factories
ALTER COLUMN name
SET NOT NULL;

