DELETE FROM role_permission_organization
WHERE permission = 'MANAGE_COURSE_ENROLLMENTS'
  AND organization_id IS NULL
  AND organization_role_id IN (
      SELECT id
      FROM organization_roles
      WHERE name IN ('SUPER_ADMIN', 'ADMIN', 'MODERATOR')
  );

UPDATE course_join_requests
SET status = 'pending',
    reviewer_user_id = NULL,
    decision_reason = NULL,
    updated_at = NOW(),
    decided_at = NULL
WHERE status = 'waitlisted';

ALTER TABLE course_join_requests
    DROP CONSTRAINT course_join_requests_status_check;

ALTER TABLE course_join_requests
    ADD CONSTRAINT course_join_requests_status_check
    CHECK (status IN ('pending', 'approved', 'rejected', 'cancelled'));

DROP INDEX course_join_requests_open_per_user_course_idx;

CREATE UNIQUE INDEX course_join_requests_one_pending_per_user_course_idx
    ON course_join_requests (course_id, requester_user_id)
    WHERE status = 'pending';
