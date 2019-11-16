-- This file should undo anything in `up.sql`

ALTER TABLE factories
ALTER COLUMN id
DROP DEFAULT;