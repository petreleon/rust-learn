-- Centralized wallet transactions migration (up)
-- Creates internal and external transaction tables, a generic transactions
-- table and many-to-many linking tables between the generic transactions
-- and the specialized tables.

-- internal transactions: server-side effects
CREATE TABLE IF NOT EXISTS internal_transactions (
	id BIGSERIAL PRIMARY KEY,
	wallet_id INT NOT NULL REFERENCES wallets(id) ON DELETE CASCADE,
	amount NUMERIC NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_internal_transactions_wallet_id ON internal_transactions (wallet_id);

-- external transactions: blockchain-side effects
CREATE TABLE IF NOT EXISTS external_transactions (
	id BIGSERIAL PRIMARY KEY,
	amount NUMERIC NOT NULL,
	blockchain_address TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_external_transactions_address ON external_transactions (blockchain_address);

-- generic transactions table
CREATE TABLE IF NOT EXISTS transactions (
	id BIGSERIAL PRIMARY KEY,
	type VARCHAR(50) NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_transactions_created_at ON transactions (created_at);

-- linking tables (many-to-many relationships)
CREATE TABLE IF NOT EXISTS transactions_internal_transactions (
	transaction_id BIGINT NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
	internal_transaction_id BIGINT NOT NULL REFERENCES internal_transactions(id) ON DELETE CASCADE,
	PRIMARY KEY (transaction_id, internal_transaction_id)
);
CREATE INDEX IF NOT EXISTS idx_transactions_internal_transactions_transaction_id ON transactions_internal_transactions (transaction_id);
CREATE INDEX IF NOT EXISTS idx_transactions_internal_transactions_internal_id ON transactions_internal_transactions (internal_transaction_id);

CREATE TABLE IF NOT EXISTS transactions_external_transactions (
	transaction_id BIGINT NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
	external_transaction_id BIGINT NOT NULL REFERENCES external_transactions(id) ON DELETE CASCADE,
	PRIMARY KEY (transaction_id, external_transaction_id)
);
CREATE INDEX IF NOT EXISTS idx_transactions_external_transactions_transaction_id ON transactions_external_transactions (transaction_id);
CREATE INDEX IF NOT EXISTS idx_transactions_external_transactions_external_id ON transactions_external_transactions (external_transaction_id);

