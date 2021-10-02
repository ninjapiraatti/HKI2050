-- Your SQL goes here

CREATE TABLE invitations (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  password_plain VARCHAR(100) NOT NULL,
  expires_at TIMESTAMP NOT NULL
);

ALTER TABLE invitations
ADD COLUMN created_by VARCHAR(100) NOT NULL,
ADD COLUMN created_at TIMESTAMP NOT NULL,
ADD COLUMN updated_by VARCHAR(100) NOT NULL,
ADD COLUMN updated_at TIMESTAMP NOT NULL,
ADD COLUMN updated_count SMALLINT NOT NULL;

SELECT hki_manage_table('users');
SELECT hki_manage_table('invitations');