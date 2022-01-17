-- Your SQL goes here

CREATE TABLE articles (
  id UUID NOT NULL PRIMARY KEY,
  character_id UUID NOT NULL,
  title VARCHAR(200) NOT NULL,
  ingress VARCHAR(1000) NOT NULL,
  body VARCHAR(50000) NOT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_articles_characters
    FOREIGN KEY (character_id)
        REFERENCES characters(id)
);

SELECT hki_manage_table('articles');