CREATE TABLE reward_compensation_records (
    id BIGSERIAL PRIMARY KEY,
    reward_candidate_id BIGINT NOT NULL REFERENCES reward_candidates(id) ON DELETE RESTRICT,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id) ON DELETE RESTRICT,
    transaction_id BIGINT NOT NULL REFERENCES transactions(id) ON DELETE RESTRICT,
    internal_transaction_id BIGINT NOT NULL REFERENCES internal_transactions(id) ON DELETE RESTRICT,
    amount NUMERIC NOT NULL,
    reason TEXT NOT NULL,
    idempotency_key TEXT NOT NULL,
    created_by_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_compensation_records_idempotency_unique UNIQUE (idempotency_key),
    CONSTRAINT reward_compensation_records_internal_unique UNIQUE (internal_transaction_id)
);

CREATE INDEX reward_compensation_records_candidate_idx
    ON reward_compensation_records (reward_candidate_id);

CREATE INDEX reward_compensation_records_wallet_idx
    ON reward_compensation_records (wallet_id);

CREATE INDEX reward_compensation_records_transaction_idx
    ON reward_compensation_records (transaction_id);

CREATE INDEX reward_compensation_records_created_by_idx
    ON reward_compensation_records (created_by_user_id);
