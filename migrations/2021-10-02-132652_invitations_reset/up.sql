-- Your SQL goes here
CREATE TABLE reset_requests (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  expires_at TIMESTAMP NOT NULL
);

alter table invitations
add column reset_request_id UUID NULL;

ALTER TABLE invitations
    ADD CONSTRAINT fk_invitations_reset_pw
    FOREIGN KEY (reset_request_id) 
    REFERENCES reset_requests (id);