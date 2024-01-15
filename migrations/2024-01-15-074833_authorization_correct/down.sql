-- This file should undo anything in `up.sql`
ALTER TABLE authorizations
RENAME COLUMN user_id TO id_user;
