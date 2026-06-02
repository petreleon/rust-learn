ALTER TABLE external_transactions
    ADD COLUMN IF NOT EXISTS chain_id BIGINT,
    ADD COLUMN IF NOT EXISTS contract_address TEXT,
    ADD COLUMN IF NOT EXISTS transaction_hash TEXT,
    ADD COLUMN IF NOT EXISTS log_index BIGINT,
    ADD COLUMN IF NOT EXISTS event_type VARCHAR(50),
    ADD COLUMN IF NOT EXISTS from_address TEXT,
    ADD COLUMN IF NOT EXISTS to_address TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_external_transactions_chain_tx_log
ON external_transactions (chain_id, transaction_hash, log_index)
WHERE chain_id IS NOT NULL
  AND transaction_hash IS NOT NULL
  AND log_index IS NOT NULL;
