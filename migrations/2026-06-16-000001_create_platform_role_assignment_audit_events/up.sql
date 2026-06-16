CREATE TABLE platform_role_assignment_audit_events (
    id BIGSERIAL PRIMARY KEY,
    target_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    actor_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    platform_role_id INTEGER REFERENCES platform_roles(id) ON DELETE SET NULL,
    role_name VARCHAR(255) NOT NULL,
    event_type VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT platform_role_assignment_audit_events_event_type_check
        CHECK (event_type IN ('role_assigned'))
);

CREATE INDEX platform_role_assignment_audit_events_target_idx
    ON platform_role_assignment_audit_events (target_user_id, created_at DESC, id DESC);

CREATE INDEX platform_role_assignment_audit_events_actor_idx
    ON platform_role_assignment_audit_events (actor_user_id)
    WHERE actor_user_id IS NOT NULL;
