-- This file should undo anything in `up.sql`
ALTER TABLE inventorybay
  DROP COLUMN friendly_name;
