CREATE TABLE course_completion_terms (
    id BIGSERIAL PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    teacher_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id INTEGER REFERENCES organizations(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    status VARCHAR(32) NOT NULL,
    completion_reward_amount NUMERIC NOT NULL,
    max_enrolled_students INTEGER NOT NULL,
    reward_policy_id BIGINT REFERENCES reward_policies(id) ON DELETE SET NULL,
    proposed_by_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    accepted_by_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    accepted_at TIMESTAMPTZ,
    activated_at TIMESTAMPTZ,
    superseded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT course_completion_terms_status_check
        CHECK (status IN (
            'submitted',
            'countered',
            'accepted',
            'active',
            'rejected',
            'withdrawn',
            'superseded'
        )),
    CONSTRAINT course_completion_terms_positive_version_check CHECK (version > 0),
    CONSTRAINT course_completion_terms_non_negative_reward_check
        CHECK (completion_reward_amount >= 0),
    CONSTRAINT course_completion_terms_positive_capacity_check
        CHECK (max_enrolled_students > 0)
);

CREATE UNIQUE INDEX course_completion_terms_course_version_idx
    ON course_completion_terms (course_id, version);

CREATE UNIQUE INDEX course_completion_terms_one_active_idx
    ON course_completion_terms (course_id)
    WHERE status = 'active';

CREATE INDEX course_completion_terms_course_status_idx
    ON course_completion_terms (course_id, status, updated_at DESC);

CREATE TABLE course_completion_term_audit_events (
    id BIGSERIAL PRIMARY KEY,
    terms_id BIGINT NOT NULL REFERENCES course_completion_terms(id) ON DELETE CASCADE,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    actor_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    event_type VARCHAR(32) NOT NULL,
    previous_status VARCHAR(32),
    new_status VARCHAR(32) NOT NULL,
    completion_reward_amount NUMERIC NOT NULL,
    max_enrolled_students INTEGER NOT NULL,
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT course_completion_term_audit_event_type_check
        CHECK (event_type IN (
            'proposed',
            'countered',
            'accepted',
            'activated',
            'rejected',
            'withdrawn',
            'superseded'
        )),
    CONSTRAINT course_completion_term_audit_previous_status_check
        CHECK (
            previous_status IS NULL
            OR previous_status IN (
                'submitted',
                'countered',
                'accepted',
                'active',
                'rejected',
                'withdrawn',
                'superseded'
            )
        ),
    CONSTRAINT course_completion_term_audit_new_status_check
        CHECK (new_status IN (
            'submitted',
            'countered',
            'accepted',
            'active',
            'rejected',
            'withdrawn',
            'superseded'
        )),
    CONSTRAINT course_completion_term_audit_non_negative_reward_check
        CHECK (completion_reward_amount >= 0),
    CONSTRAINT course_completion_term_audit_positive_capacity_check
        CHECK (max_enrolled_students > 0)
);

CREATE INDEX course_completion_term_audit_events_course_idx
    ON course_completion_term_audit_events (course_id, created_at DESC);
