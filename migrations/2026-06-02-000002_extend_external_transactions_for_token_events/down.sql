DROP INDEX IF EXISTS idx_external_transactions_chain_tx_log;

ALTER TABLE external_transactions
    DROP COLUMN IF EXISTS to_address,
    DROP COLUMN IF EXISTS from_address,
    DROP COLUMN IF EXISTS event_type,
    DROP COLUMN IF EXISTS log_index,
    DROP COLUMN IF EXISTS transaction_hash,
    DROP COLUMN IF EXISTS contract_address,
    DROP COLUMN IF EXISTS chain_id;
