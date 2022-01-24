ALTER TABLE articles
RENAME COLUMN inserted_at TO created_at;
ALTER TABLE articles
RENAME COLUMN inserted_by TO created_by;
ALTER TABLE tags
RENAME COLUMN inserted_at TO created_at;
ALTER TABLE tags
RENAME COLUMN inserted_by TO created_by;
ALTER TABLE contenttags
RENAME COLUMN inserted_at TO created_at;
ALTER TABLE contenttags
RENAME COLUMN inserted_by TO created_by;
