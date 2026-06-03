CREATE TABLE reward_audit_events (
    id BIGSERIAL PRIMARY KEY,
    reward_candidate_id BIGINT NOT NULL REFERENCES reward_candidates(id) ON DELETE CASCADE,
    actor_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    event_type VARCHAR(64) NOT NULL,
    from_status VARCHAR(32),
    to_status VARCHAR(32) NOT NULL,
    reason TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_audit_events_event_type_check
        CHECK (event_type IN (
            'candidate_submitted',
            'teacher_decision',
            'amount_decision',
            'token_confirmed',
            'wallet_credited',
            'wallet_credit_notified',
            'reconciled'
        )),
    CONSTRAINT reward_audit_events_from_status_check
        CHECK (from_status IS NULL OR from_status IN (
            'pending_teacher_approval',
            'teacher_approved',
            'teacher_rejected',
            'amount_approved',
            'amount_rejected',
            'adjusted',
            'token_pending',
            'token_confirmed',
            'wallet_credited',
            'notified',
            'completed',
            'needs_reconciliation',
            'failed'
        )),
    CONSTRAINT reward_audit_events_to_status_check
        CHECK (to_status IN (
            'pending_teacher_approval',
            'teacher_approved',
            'teacher_rejected',
            'amount_approved',
            'amount_rejected',
            'adjusted',
            'token_pending',
            'token_confirmed',
            'wallet_credited',
            'notified',
            'completed',
            'needs_reconciliation',
            'failed'
        ))
);

CREATE INDEX reward_audit_events_candidate_idx
    ON reward_audit_events (reward_candidate_id, created_at);

CREATE INDEX reward_audit_events_actor_idx
    ON reward_audit_events (actor_user_id)
    WHERE actor_user_id IS NOT NULL;
