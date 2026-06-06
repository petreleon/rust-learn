CREATE TABLE wallet_token_deposit_intents (
    id BIGSERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id) ON DELETE RESTRICT,
    ethereum_address TEXT NOT NULL,
    platform_address TEXT NOT NULL,
    amount NUMERIC NOT NULL,
    gas_payer VARCHAR(32) NOT NULL,
    tax_amount NUMERIC NOT NULL DEFAULT 0,
    status VARCHAR(32) NOT NULL DEFAULT 'pending_chain_confirmation',
    chain_id BIGINT,
    contract_address TEXT,
    transaction_hash TEXT,
    log_index BIGINT,
    event_type VARCHAR(50),
    external_transaction_id BIGINT REFERENCES external_transactions(id) ON DELETE SET NULL,
    transaction_id BIGINT REFERENCES transactions(id) ON DELETE SET NULL,
    wallet_provider VARCHAR(32) NOT NULL DEFAULT 'metamask',
    metamask_required BOOLEAN NOT NULL DEFAULT TRUE,
    wallet_action VARCHAR(64) NOT NULL,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    credited_at TIMESTAMPTZ,
    CONSTRAINT wallet_token_deposit_intents_amount_check CHECK (amount > 0),
    CONSTRAINT wallet_token_deposit_intents_tax_check CHECK (tax_amount >= 0),
    CONSTRAINT wallet_token_deposit_intents_gas_payer_check CHECK (gas_payer IN ('user', 'platform')),
    CONSTRAINT wallet_token_deposit_intents_status_check CHECK (
        status IN (
            'pending_chain_confirmation',
            'credited',
            'ambiguous',
            'failed'
        )
    ),
    CONSTRAINT wallet_token_deposit_intents_log_index_check CHECK (log_index IS NULL OR log_index >= 0),
    CONSTRAINT wallet_token_deposit_intents_chain_check CHECK (chain_id IS NULL OR chain_id > 0)
);

CREATE UNIQUE INDEX wallet_token_deposit_intents_chain_log_unique_idx
    ON wallet_token_deposit_intents (chain_id, transaction_hash, log_index)
    WHERE chain_id IS NOT NULL
      AND transaction_hash IS NOT NULL
      AND log_index IS NOT NULL
      AND status != 'ambiguous';

CREATE INDEX wallet_token_deposit_intents_pending_idx
    ON wallet_token_deposit_intents (status, created_at)
    WHERE status = 'pending_chain_confirmation';

CREATE INDEX wallet_token_deposit_intents_user_idx
    ON wallet_token_deposit_intents (user_id, created_at DESC);

CREATE INDEX wallet_token_deposit_intents_wallet_idx
    ON wallet_token_deposit_intents (wallet_id, created_at DESC);
