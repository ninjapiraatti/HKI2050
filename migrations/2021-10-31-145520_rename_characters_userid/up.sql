-- Your SQL goes here
ALTER TABLE characters
RENAME COLUMN userid TO user_id;

ALTER TABLE characters DROP CONSTRAINT fk_characters_users;

ALTER TABLE characters
    ADD CONSTRAINT fk_characters_users
    FOREIGN KEY (user_id)
    REFERENCES users (id);