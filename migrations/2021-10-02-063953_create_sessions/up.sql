-- Your SQL goes here


-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION hki_manage_table(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER hki_set_created BEFORE INSERT ON %s
                    FOR EACH ROW EXECUTE PROCEDURE hki_set_created()', _tbl);
    EXECUTE format('CREATE TRIGGER hki_set_updated BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE hki_set_updated()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION hki_set_created() RETURNS trigger AS $$
BEGIN
    NEW.created_at := current_timestamp;
    NEW.updated_at := NEW.created_at;
    NEW.created_by := NEW.updated_by;
    NEW.updated_count := 0;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION hki_set_updated() RETURNS trigger AS $$
BEGIN
    IF (NEW IS DISTINCT FROM OLD) THEN
        NEW.created_by := OLD.created_by;
        NEW.created_at := OLD.created_at;
        NEW.updated_at := current_timestamp;
        NEW.updated_count := OLD.updated_count + 1;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE sessions (
    id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    expire_at TIMESTAMP NOT NULL,
    created_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_sessions_users
    FOREIGN KEY (user_id)
        REFERENCES users(id)
 ON DELETE CASCADE

);
Select hki_manage_table('sessions');

CREATE VIEW activesessions AS
select 
	s.id "session_id",
	u.id "user_id",
	u.email "email",
	s.expire_at "expire_at",
	u.isadmin "isadmin"
from 
	users u, 
	sessions s
where u.id = s.user_id;
