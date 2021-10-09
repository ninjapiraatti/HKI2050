-- Your SQL goes here

CREATE TABLE characters (
  id UUID NOT NULL PRIMARY KEY,
  userid UUID NOT NULL,
  name VARCHAR(100) NOT NULL,
  description VARCHAR(1000) NOT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_characters_users
    FOREIGN KEY (userid)
        REFERENCES users(id)
);

SELECT hki_manage_table('characters');