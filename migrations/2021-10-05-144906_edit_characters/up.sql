-- Your SQL goes here
ALTER TABLE characters
RENAME COLUMN inserted_at TO created_at;
ALTER TABLE characters
RENAME COLUMN inserted_by TO created_by;