CREATE TABLE reward_wallet_credit_records (
    id BIGSERIAL PRIMARY KEY,
    reward_candidate_id BIGINT NOT NULL REFERENCES reward_candidates(id) ON DELETE CASCADE,
    wallet_id INTEGER NOT NULL REFERENCES wallets(id) ON DELETE RESTRICT,
    transaction_id BIGINT NOT NULL REFERENCES transactions(id) ON DELETE RESTRICT,
    internal_transaction_id BIGINT NOT NULL REFERENCES internal_transactions(id) ON DELETE RESTRICT,
    notification_id BIGINT REFERENCES notifications(id) ON DELETE SET NULL,
    notified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_wallet_credit_records_candidate_unique UNIQUE (reward_candidate_id),
    CONSTRAINT reward_wallet_credit_records_internal_unique UNIQUE (internal_transaction_id),
    CONSTRAINT reward_wallet_credit_records_notification_unique UNIQUE (notification_id)
);

CREATE INDEX reward_wallet_credit_records_wallet_idx
    ON reward_wallet_credit_records (wallet_id);

CREATE INDEX reward_wallet_credit_records_transaction_idx
    ON reward_wallet_credit_records (transaction_id);
