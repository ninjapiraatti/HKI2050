-- Your SQL goes here
ALTER TABLE articles
DROP CONSTRAINT fk_articles_characters;

ALTER TABLE articles
  ADD CONSTRAINT fk_articles_users
    FOREIGN KEY (user_id)
        REFERENCES users(id);