-- Your SQL goes here
ALTER TABLE inventorybay
  ADD COLUMN friendly_name VARCHAR(191)
  NOT NULL
  DEFAULT 'default';

UPDATE inventorybay SET friendly_name = name WHERE friendly_name = 'default';
