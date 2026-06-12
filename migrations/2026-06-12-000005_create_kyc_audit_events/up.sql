CREATE TABLE kyc_audit_events (
    id BIGSERIAL PRIMARY KEY,
    submission_id BIGINT NOT NULL REFERENCES kyc_submissions(id) ON DELETE CASCADE,
    actor_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    event_type VARCHAR(64) NOT NULL,
    from_status VARCHAR(32),
    to_status VARCHAR(32) NOT NULL,
    reason TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT kyc_audit_events_event_type_check
        CHECK (event_type IN ('submitted', 'review_decision')),
    CONSTRAINT kyc_audit_events_from_status_check
        CHECK (from_status IS NULL OR from_status IN (
            'submitted', 'under_review', 'verified', 'rejected', 'expired', 'provider_error'
        )),
    CONSTRAINT kyc_audit_events_to_status_check
        CHECK (to_status IN (
            'submitted', 'under_review', 'verified', 'rejected', 'expired', 'provider_error'
        ))
);

CREATE INDEX kyc_audit_events_submission_idx
    ON kyc_audit_events (submission_id, created_at, id);

CREATE INDEX kyc_audit_events_actor_idx
    ON kyc_audit_events (actor_user_id)
    WHERE actor_user_id IS NOT NULL;
