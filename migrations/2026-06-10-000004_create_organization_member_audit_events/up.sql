CREATE TABLE organization_member_audit_events (
    id BIGSERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id),
    actor_user_id INTEGER REFERENCES users(id),
    target_user_id INTEGER NOT NULL REFERENCES users(id),
    event_type VARCHAR(64) NOT NULL,
    role_name VARCHAR(64),
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_org_member_audit_org ON organization_member_audit_events(organization_id);
CREATE INDEX idx_org_member_audit_user ON organization_member_audit_events(target_user_id);
