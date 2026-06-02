CREATE TABLE course_join_requests (
    id BIGSERIAL PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    requester_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    reviewer_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    decision_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    decided_at TIMESTAMPTZ,
    CONSTRAINT course_join_requests_status_check
        CHECK (status IN ('pending', 'approved', 'rejected', 'cancelled'))
);

CREATE UNIQUE INDEX course_join_requests_one_pending_per_user_course_idx
    ON course_join_requests (course_id, requester_user_id)
    WHERE status = 'pending';

CREATE INDEX course_join_requests_course_status_idx
    ON course_join_requests (course_id, status);

CREATE INDEX course_join_requests_requester_idx
    ON course_join_requests (requester_user_id);

WITH roles AS (
    SELECT id, name
    FROM organization_roles
    WHERE name IN ('SUPER_ADMIN', 'ADMIN', 'MODERATOR')
), pairs AS (
    SELECT r.id AS organization_role_id, v.permission
    FROM roles r
    JOIN (
        VALUES
            ('SUPER_ADMIN', 'APPROVE_COURSE_JOIN_REQUESTS'),
            ('ADMIN', 'APPROVE_COURSE_JOIN_REQUESTS'),
            ('MODERATOR', 'APPROVE_COURSE_JOIN_REQUESTS')
    ) AS v(role_name, permission)
    ON v.role_name = r.name
)
INSERT INTO role_permission_organization (organization_id, organization_role_id, permission)
SELECT NULL::INT, organization_role_id, permission
FROM pairs
WHERE NOT EXISTS (
    SELECT 1
    FROM role_permission_organization rpo
    WHERE rpo.organization_id IS NULL
      AND rpo.organization_role_id = pairs.organization_role_id
      AND rpo.permission = pairs.permission
);
