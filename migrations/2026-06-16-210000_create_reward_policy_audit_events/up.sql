CREATE TABLE reward_policy_audit_events (
    id BIGSERIAL PRIMARY KEY,
    reward_policy_id BIGINT NOT NULL REFERENCES reward_policies(id) ON DELETE CASCADE,
    actor_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    event_type VARCHAR(32) NOT NULL,
    previous_active BOOLEAN,
    new_active BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_policy_audit_events_event_type_check
        CHECK (event_type IN ('created', 'activated', 'deactivated'))
);

CREATE INDEX reward_policy_audit_events_policy_idx
    ON reward_policy_audit_events (reward_policy_id, created_at DESC, id DESC);

CREATE INDEX reward_policy_audit_events_actor_idx
    ON reward_policy_audit_events (actor_user_id)
    WHERE actor_user_id IS NOT NULL;
