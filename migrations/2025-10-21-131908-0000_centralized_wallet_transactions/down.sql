-- Reverse migration for centralized wallet transactions

DROP INDEX IF EXISTS idx_transactions_external_transactions_external_id;
DROP INDEX IF EXISTS idx_transactions_external_transactions_transaction_id;
DROP TABLE IF EXISTS transactions_external_transactions;

DROP INDEX IF EXISTS idx_transactions_internal_transactions_internal_id;
DROP INDEX IF EXISTS idx_transactions_internal_transactions_transaction_id;
DROP TABLE IF EXISTS transactions_internal_transactions;

DROP INDEX IF EXISTS idx_transactions_created_at;
DROP TABLE IF EXISTS transactions;

DROP INDEX IF EXISTS idx_external_transactions_address;
DROP TABLE IF EXISTS external_transactions;

DROP INDEX IF EXISTS idx_internal_transactions_wallet_id;
DROP TABLE IF EXISTS internal_transactions;

