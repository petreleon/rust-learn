
-- Minimal wallets migration
-- This migration creates a minimal `wallets` table that stores a numeric
-- `value` associated with a `user_id` and an index on `user_id`.

CREATE TABLE IF NOT EXISTS wallets (
	id SERIAL PRIMARY KEY,
	user_id INT NULL REFERENCES users(id) ON DELETE CASCADE,
	organization_id INT NULL REFERENCES organizations(id) ON DELETE CASCADE,
	value NUMERIC NOT NULL DEFAULT 0,
	-- enforce exactly one of user_id or organization_id is non-null
	CONSTRAINT wallets_one_owner CHECK (
		(user_id IS NULL AND organization_id IS NOT NULL) OR
		(user_id IS NOT NULL AND organization_id IS NULL)
	)
);

-- indexes for fast lookups by owner
CREATE INDEX IF NOT EXISTS wallets_user_id_idx ON wallets (user_id);
CREATE INDEX IF NOT EXISTS wallets_organization_id_idx ON wallets (organization_id);


