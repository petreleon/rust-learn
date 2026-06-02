CREATE TABLE reward_payout_records (
    id BIGSERIAL PRIMARY KEY,
    reward_candidate_id BIGINT NOT NULL REFERENCES reward_candidates(id) ON DELETE CASCADE,
    transaction_id BIGINT NOT NULL REFERENCES transactions(id) ON DELETE RESTRICT,
    external_transaction_id BIGINT NOT NULL REFERENCES external_transactions(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_payout_records_candidate_unique UNIQUE (reward_candidate_id),
    CONSTRAINT reward_payout_records_external_unique UNIQUE (external_transaction_id)
);

CREATE INDEX reward_payout_records_transaction_idx
    ON reward_payout_records (transaction_id);
