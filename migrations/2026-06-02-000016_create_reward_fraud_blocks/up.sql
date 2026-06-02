CREATE TABLE reward_fraud_blocks (
    id BIGSERIAL PRIMARY KEY,
    scope_type VARCHAR(32) NOT NULL,
    teacher_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    organization_id INTEGER REFERENCES organizations(id) ON DELETE CASCADE,
    course_id INTEGER REFERENCES courses(id) ON DELETE CASCADE,
    reward_policy_id BIGINT REFERENCES reward_policies(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    evidence_reference TEXT,
    created_by_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    expires_at TIMESTAMPTZ,
    revoked_by_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_fraud_blocks_scope_type_check
        CHECK (scope_type IN ('teacher', 'organization', 'course', 'reward_policy')),
    CONSTRAINT reward_fraud_blocks_scope_target_check
        CHECK (
            (scope_type = 'teacher' AND teacher_user_id IS NOT NULL AND organization_id IS NULL AND course_id IS NULL AND reward_policy_id IS NULL)
            OR (scope_type = 'organization' AND teacher_user_id IS NULL AND organization_id IS NOT NULL AND course_id IS NULL AND reward_policy_id IS NULL)
            OR (scope_type = 'course' AND teacher_user_id IS NULL AND organization_id IS NULL AND course_id IS NOT NULL AND reward_policy_id IS NULL)
            OR (scope_type = 'reward_policy' AND teacher_user_id IS NULL AND organization_id IS NULL AND course_id IS NULL AND reward_policy_id IS NOT NULL)
        ),
    CONSTRAINT reward_fraud_blocks_revocation_check
        CHECK (
            (revoked_at IS NULL AND revoked_by_user_id IS NULL)
            OR (revoked_at IS NOT NULL AND revoked_by_user_id IS NOT NULL)
        )
);

CREATE UNIQUE INDEX reward_fraud_blocks_active_teacher_idx
    ON reward_fraud_blocks (teacher_user_id)
    WHERE scope_type = 'teacher' AND revoked_at IS NULL;

CREATE UNIQUE INDEX reward_fraud_blocks_active_organization_idx
    ON reward_fraud_blocks (organization_id)
    WHERE scope_type = 'organization' AND revoked_at IS NULL;

CREATE UNIQUE INDEX reward_fraud_blocks_active_course_idx
    ON reward_fraud_blocks (course_id)
    WHERE scope_type = 'course' AND revoked_at IS NULL;

CREATE UNIQUE INDEX reward_fraud_blocks_active_policy_idx
    ON reward_fraud_blocks (reward_policy_id)
    WHERE scope_type = 'reward_policy' AND revoked_at IS NULL;

CREATE INDEX reward_fraud_blocks_active_idx
    ON reward_fraud_blocks (scope_type, revoked_at, expires_at);
