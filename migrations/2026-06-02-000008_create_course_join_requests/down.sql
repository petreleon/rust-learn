DELETE FROM role_permission_organization
WHERE permission = 'APPROVE_COURSE_JOIN_REQUESTS'
  AND organization_id IS NULL
  AND organization_role_id IN (
      SELECT id
      FROM organization_roles
      WHERE name IN ('SUPER_ADMIN', 'ADMIN', 'MODERATOR')
  );

DROP TABLE IF EXISTS course_join_requests;
