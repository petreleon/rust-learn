-- Seed test-related permissions for platform roles (idempotent)
-- Permissions added correspond to enum variants in src/config/constants/permissions.rs
--   RUN_TESTS, VIEW_TEST_RESULTS, MANAGE_TEST_SUITES
-- Mapped to roles:
--   SUPER_ADMIN: RUN_TESTS, VIEW_TEST_RESULTS, MANAGE_TEST_SUITES
--   ADMIN:       RUN_TESTS, VIEW_TEST_RESULTS, MANAGE_TEST_SUITES
--   MODERATOR:   VIEW_TEST_RESULTS
--   TEACHER:     RUN_TESTS, VIEW_TEST_RESULTS

WITH roles AS (
    SELECT id, name FROM platform_roles
    WHERE name IN ('SUPER_ADMIN','ADMIN','MODERATOR','TEACHER')
), pairs AS (
    SELECT r.id AS platform_role_id, p.permission
    FROM roles r
    JOIN (
        VALUES
            ('SUPER_ADMIN','RUN_TESTS'),
            ('SUPER_ADMIN','VIEW_TEST_RESULTS'),
            ('SUPER_ADMIN','MANAGE_TEST_SUITES'),
            ('ADMIN','RUN_TESTS'),
            ('ADMIN','VIEW_TEST_RESULTS'),
            ('ADMIN','MANAGE_TEST_SUITES'),
            ('MODERATOR','VIEW_TEST_RESULTS'),
            ('TEACHER','RUN_TESTS'),
            ('TEACHER','VIEW_TEST_RESULTS')
    ) AS p(role_name, permission)
    ON p.role_name = r.name
)
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT platform_role_id, permission
FROM pairs
WHERE NOT EXISTS (
    SELECT 1 FROM role_permission_platform rpp
    WHERE rpp.platform_role_id = pairs.platform_role_id
      AND rpp.permission = pairs.permission
);
