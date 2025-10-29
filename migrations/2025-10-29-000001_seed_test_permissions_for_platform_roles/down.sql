-- Rollback seed of test-related permissions for platform roles
-- This removes only the permissions introduced by the matching up.sql.

DELETE FROM role_permission_platform rpp
USING platform_roles pr
WHERE rpp.platform_role_id = pr.id
  AND pr.name IN ('SUPER_ADMIN','ADMIN','MODERATOR','TEACHER')
  AND rpp.permission IN ('RUN_TESTS','VIEW_TEST_RESULTS','MANAGE_TEST_SUITES');
