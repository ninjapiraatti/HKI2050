-- Your SQL goes here

CREATE TABLE contenttags (
  id UUID NOT NULL PRIMARY KEY,
  tag_id UUID NOT NULL,
  content_id UUID NOT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL
);

SELECT hki_manage_table('contenttags');
