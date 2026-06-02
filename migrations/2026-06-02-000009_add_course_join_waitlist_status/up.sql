ALTER TABLE course_join_requests
    DROP CONSTRAINT course_join_requests_status_check;

ALTER TABLE course_join_requests
    ADD CONSTRAINT course_join_requests_status_check
    CHECK (status IN ('pending', 'waitlisted', 'approved', 'rejected', 'cancelled'));

DROP INDEX course_join_requests_one_pending_per_user_course_idx;

CREATE UNIQUE INDEX course_join_requests_open_per_user_course_idx
    ON course_join_requests (course_id, requester_user_id)
    WHERE status IN ('pending', 'waitlisted');

WITH roles AS (
    SELECT id, name
    FROM organization_roles
    WHERE name IN ('SUPER_ADMIN', 'ADMIN', 'MODERATOR')
), pairs AS (
    SELECT r.id AS organization_role_id, v.permission
    FROM roles r
    JOIN (
        VALUES
            ('SUPER_ADMIN', 'MANAGE_COURSE_ENROLLMENTS'),
            ('ADMIN', 'MANAGE_COURSE_ENROLLMENTS'),
            ('MODERATOR', 'MANAGE_COURSE_ENROLLMENTS')
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
