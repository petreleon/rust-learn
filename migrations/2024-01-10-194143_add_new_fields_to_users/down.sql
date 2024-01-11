-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN date_of_birth,
DROP COLUMN created_at,
DROP COLUMN kyc_verified;
