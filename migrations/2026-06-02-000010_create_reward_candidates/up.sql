CREATE TABLE reward_candidates (
    id BIGSERIAL PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    student_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    submitter_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    source_scope VARCHAR(32) NOT NULL DEFAULT 'course',
    source_organization_id INTEGER REFERENCES organizations(id) ON DELETE SET NULL,
    event_type VARCHAR(64) NOT NULL,
    idempotency_key TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}'::jsonb,
    status VARCHAR(32) NOT NULL DEFAULT 'pending_teacher_approval',
    teacher_approver_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    teacher_decision_reason TEXT,
    teacher_decided_at TIMESTAMPTZ,
    amount_reviewer_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    approved_amount NUMERIC,
    amount_decision_reason TEXT,
    amount_decided_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_candidates_source_scope_check
        CHECK (source_scope IN ('course', 'organization')),
    CONSTRAINT reward_candidates_event_type_check
        CHECK (event_type IN (
            'assessment_completion',
            'course_completion',
            'manual_completion',
            'administrative_adjustment'
        )),
    CONSTRAINT reward_candidates_status_check
        CHECK (status IN (
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
    CONSTRAINT reward_candidates_non_negative_amount_check
        CHECK (approved_amount IS NULL OR approved_amount >= 0)
);

CREATE UNIQUE INDEX reward_candidates_idempotency_key_idx
    ON reward_candidates (idempotency_key);

CREATE INDEX reward_candidates_course_status_idx
    ON reward_candidates (course_id, status);

CREATE INDEX reward_candidates_student_idx
    ON reward_candidates (student_user_id);

CREATE INDEX reward_candidates_source_organization_idx
    ON reward_candidates (source_organization_id)
    WHERE source_organization_id IS NOT NULL;
