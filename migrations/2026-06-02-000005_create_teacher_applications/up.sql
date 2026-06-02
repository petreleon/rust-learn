CREATE TABLE teacher_applications (
    id BIGSERIAL PRIMARY KEY,
    applicant_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    requested_scope VARCHAR(32) NOT NULL,
    requested_organization_id INTEGER REFERENCES organizations(id) ON DELETE SET NULL,
    requested_course_id INTEGER REFERENCES courses(id) ON DELETE SET NULL,
    experience_summary TEXT NOT NULL,
    organization_sponsor_id INTEGER REFERENCES organizations(id) ON DELETE SET NULL,
    portfolio_links JSONB NOT NULL DEFAULT '[]'::jsonb,
    status VARCHAR(32) NOT NULL DEFAULT 'submitted',
    reviewer_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    decision_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    decided_at TIMESTAMPTZ,
    CONSTRAINT teacher_applications_requested_scope_check
        CHECK (requested_scope IN ('platform', 'organization', 'course')),
    CONSTRAINT teacher_applications_status_check
        CHECK (status IN ('submitted', 'needs_changes', 'approved', 'rejected')),
    CONSTRAINT teacher_applications_experience_summary_check
        CHECK (length(btrim(experience_summary)) > 0),
    CONSTRAINT teacher_applications_portfolio_links_array_check
        CHECK (jsonb_typeof(portfolio_links) = 'array')
);

CREATE INDEX teacher_applications_applicant_idx
    ON teacher_applications (applicant_user_id);
CREATE INDEX teacher_applications_status_idx
    ON teacher_applications (status);
CREATE INDEX teacher_applications_sponsor_idx
    ON teacher_applications (organization_sponsor_id);
CREATE INDEX teacher_applications_requested_org_idx
    ON teacher_applications (requested_organization_id);
CREATE INDEX teacher_applications_requested_course_idx
    ON teacher_applications (requested_course_id);

CREATE TABLE teacher_application_audit_events (
    id BIGSERIAL PRIMARY KEY,
    application_id BIGINT NOT NULL REFERENCES teacher_applications(id) ON DELETE CASCADE,
    actor_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    event_type VARCHAR(64) NOT NULL,
    from_status VARCHAR(32),
    to_status VARCHAR(32) NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT teacher_application_audit_events_event_type_check
        CHECK (length(btrim(event_type)) > 0),
    CONSTRAINT teacher_application_audit_events_from_status_check
        CHECK (
            from_status IS NULL
            OR from_status IN ('submitted', 'needs_changes', 'approved', 'rejected')
        ),
    CONSTRAINT teacher_application_audit_events_to_status_check
        CHECK (to_status IN ('submitted', 'needs_changes', 'approved', 'rejected'))
);

CREATE INDEX teacher_application_audit_events_application_idx
    ON teacher_application_audit_events (application_id);
CREATE INDEX teacher_application_audit_events_actor_idx
    ON teacher_application_audit_events (actor_user_id);
