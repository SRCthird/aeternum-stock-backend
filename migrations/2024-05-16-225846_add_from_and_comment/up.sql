-- Your SQL goes here
ALTER TABLE inventory
  ADD COLUMN from_location varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL;
ALTER TABLE inventory
  ADD COLUMN comments varchar(191) COLLATE utf8mb4_unicode_ci NOT NULL;
