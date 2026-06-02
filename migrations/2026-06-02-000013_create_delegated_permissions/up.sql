CREATE TABLE delegated_permissions (
    id BIGSERIAL PRIMARY KEY,
    grantor_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    grantee_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission VARCHAR(128) NOT NULL,
    scope_type VARCHAR(32) NOT NULL,
    organization_id INTEGER REFERENCES organizations(id) ON DELETE CASCADE,
    course_id INTEGER REFERENCES courses(id) ON DELETE CASCADE,
    reason TEXT,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revoked_by_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    revoke_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT delegated_permissions_scope_check
        CHECK (
            (scope_type = 'platform' AND organization_id IS NULL AND course_id IS NULL)
            OR (scope_type = 'organization' AND organization_id IS NOT NULL AND course_id IS NULL)
            OR (scope_type = 'course' AND organization_id IS NULL AND course_id IS NOT NULL)
        ),
    CONSTRAINT delegated_permissions_expiration_check
        CHECK (expires_at IS NULL OR expires_at > created_at)
);

CREATE INDEX delegated_permissions_active_platform_idx
    ON delegated_permissions (grantee_user_id, permission, scope_type)
    WHERE revoked_at IS NULL AND scope_type = 'platform';

CREATE INDEX delegated_permissions_active_organization_idx
    ON delegated_permissions (grantee_user_id, permission, organization_id)
    WHERE revoked_at IS NULL AND scope_type = 'organization';

CREATE INDEX delegated_permissions_active_course_idx
    ON delegated_permissions (grantee_user_id, permission, course_id)
    WHERE revoked_at IS NULL AND scope_type = 'course';
