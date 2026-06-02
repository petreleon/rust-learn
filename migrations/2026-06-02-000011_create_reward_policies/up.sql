CREATE TABLE reward_policies (
    id BIGSERIAL PRIMARY KEY,
    scope_type VARCHAR(32) NOT NULL,
    organization_id INTEGER REFERENCES organizations(id) ON DELETE CASCADE,
    course_id INTEGER REFERENCES courses(id) ON DELETE CASCADE,
    event_type VARCHAR(64) NOT NULL,
    version INTEGER NOT NULL,
    token_amount NUMERIC NOT NULL,
    multiplier NUMERIC NOT NULL DEFAULT 1,
    max_payout NUMERIC,
    cooldown_seconds BIGINT NOT NULL DEFAULT 0,
    payment_strategy VARCHAR(32) NOT NULL DEFAULT 'treasury_transfer',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_policies_scope_type_check
        CHECK (scope_type IN ('platform', 'organization', 'course')),
    CONSTRAINT reward_policies_scope_shape_check
        CHECK (
            (scope_type = 'platform' AND organization_id IS NULL AND course_id IS NULL)
            OR (scope_type = 'organization' AND organization_id IS NOT NULL AND course_id IS NULL)
            OR (scope_type = 'course' AND course_id IS NOT NULL)
        ),
    CONSTRAINT reward_policies_event_type_check
        CHECK (event_type IN (
            'assessment_completion',
            'course_completion',
            'manual_completion',
            'administrative_adjustment'
        )),
    CONSTRAINT reward_policies_payment_strategy_check
        CHECK (payment_strategy IN ('treasury_transfer', 'mint', 'off_chain')),
    CONSTRAINT reward_policies_positive_version_check
        CHECK (version > 0),
    CONSTRAINT reward_policies_non_negative_amount_check
        CHECK (token_amount >= 0),
    CONSTRAINT reward_policies_positive_multiplier_check
        CHECK (multiplier > 0),
    CONSTRAINT reward_policies_non_negative_max_payout_check
        CHECK (max_payout IS NULL OR max_payout >= 0),
    CONSTRAINT reward_policies_non_negative_cooldown_check
        CHECK (cooldown_seconds >= 0)
);

CREATE UNIQUE INDEX reward_policies_version_idx
    ON reward_policies (
        scope_type,
        COALESCE(organization_id, 0),
        COALESCE(course_id, 0),
        event_type,
        version
    );

CREATE UNIQUE INDEX reward_policies_one_active_idx
    ON reward_policies (
        scope_type,
        COALESCE(organization_id, 0),
        COALESCE(course_id, 0),
        event_type
    )
    WHERE active = TRUE;

CREATE INDEX reward_policies_scope_event_idx
    ON reward_policies (scope_type, event_type, active);
