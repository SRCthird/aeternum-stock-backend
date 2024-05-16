-- This file should undo anything in `up.sql`
ALTER TABLE inventory
  DROP COLUMN from_location;
ALTER TABLE inventory
  DROP COLUMN comments;
