-- Your SQL goes here

CREATE TABLE tags (
  id UUID NOT NULL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL
);

SELECT hki_manage_table('tags');
