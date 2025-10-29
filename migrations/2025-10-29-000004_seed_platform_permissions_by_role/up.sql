-- Seed curated platform permissions per role (idempotent)
-- Roles covered: ADMIN, MODERATOR, TEACHER, USER, STUDENT, GUEST
-- SUPER_ADMIN already receives all permissions in a prior migration.
-- Note: TAKE_TESTS is intentionally NOT granted at platform level; it's seeded at course level.

WITH roles AS (
    SELECT id, name FROM platform_roles
    WHERE name IN ('ADMIN','MODERATOR','TEACHER','USER','STUDENT','GUEST')
), pairs AS (
    -- Map each role to a curated set of platform-scoped permissions
    SELECT r.id AS platform_role_id, v.permission
    FROM roles r
    JOIN (
        -- ADMIN: broad platform management, but no impersonation
        VALUES
            ('ADMIN','MANAGE_PLATFORM_SETTINGS'),
            ('ADMIN','MANAGE_API_KEYS'),
            ('ADMIN','MANAGE_INTEGRATIONS'),
            ('ADMIN','VIEW_AUDIT_LOGS'),
            ('ADMIN','MANAGE_BILLING'),
            ('ADMIN','EXPORT_DATA'),
            ('ADMIN','RUN_MAINTENANCE_TASKS'),
            ('ADMIN','MANAGE_ROLE_PERMISSIONS'),
            ('ADMIN','VIEW_ROLE_ASSIGNMENTS'),
            ('ADMIN','MANAGE_NOTIFICATION_TEMPLATES'),
            ('ADMIN','SEND_BULK_NOTIFICATION'),
            ('ADMIN','SEND_NOTIFICATION'),
            ('ADMIN','VIEW_NOTIFICATION'),
            ('ADMIN','VIEW_ANALYTICS_DASHBOARD'),
            ('ADMIN','MANAGE_EXPORT_JOBS'),
            ('ADMIN','MANAGE_DISCUSSIONS'),
            ('ADMIN','MODERATE_DISCUSSION'),
            ('ADMIN','POST_IN_DISCUSSION'),
            ('ADMIN','VIEW_REPORT'),
            ('ADMIN','GENERATE_REPORT'),
            -- Wallets/payments oversight (no impersonation/ownership implied)
            ('ADMIN','MANAGE_WALLETS'),
            ('ADMIN','MANAGE_PAYMENT_METHODS'),
            ('ADMIN','VIEW_FINANCIAL_REPORTS'),
            ('ADMIN','VIEW_TRANSACTIONS'),
            ('ADMIN','APPROVE_CENTRALIZED_TRANSFER'),
            ('ADMIN','RECONCILE_WALLETS'),
            -- Tests (platform-scoped management)
            ('ADMIN','RUN_TESTS'),
            ('ADMIN','VIEW_TEST_RESULTS'),
            ('ADMIN','MANAGE_TEST_SUITES'),

        -- MODERATOR: moderation and limited comms/visibility
            ('MODERATOR','MANAGE_DISCUSSIONS'),
            ('MODERATOR','MODERATE_DISCUSSION'),
            ('MODERATOR','POST_IN_DISCUSSION'),
            ('MODERATOR','VIEW_NOTIFICATION'),
            ('MODERATOR','SEND_NOTIFICATION'),
            ('MODERATOR','VIEW_REPORT'),
            ('MODERATOR','VIEW_USER'),
            ('MODERATOR','VIEW_TEST_RESULTS'),

        -- TEACHER: announcements, reporting, and test runs/results
            ('TEACHER','POST_IN_DISCUSSION'),
            ('TEACHER','SEND_NOTIFICATION'),
            ('TEACHER','VIEW_NOTIFICATION'),
            ('TEACHER','VIEW_REPORT'),
            ('TEACHER','GENERATE_REPORT'),
            ('TEACHER','RUN_TESTS'),
            ('TEACHER','VIEW_TEST_RESULTS'),

        -- USER: basic participation and requests
            ('USER','VIEW_NOTIFICATION'),
            ('USER','POST_IN_DISCUSSION'),
            ('USER','VIEW_COURSE'),
            ('USER','VIEW_CONTENT'),
            ('USER','REQUEST_JOIN_COURSE'),
            ('USER','REQUEST_JOIN_ORGANIZATION'),
            ('USER','JOIN_COURSE'),

        -- STUDENT: similar to user at platform scope
            ('STUDENT','VIEW_NOTIFICATION'),
            ('STUDENT','POST_IN_DISCUSSION'),
            ('STUDENT','VIEW_COURSE'),
            ('STUDENT','VIEW_CONTENT'),
            ('STUDENT','REQUEST_JOIN_COURSE'),
            ('STUDENT','JOIN_COURSE'),

        -- GUEST: read-mostly for public resources
            ('GUEST','VIEW_NOTIFICATION'),
            ('GUEST','VIEW_COURSE'),
            ('GUEST','VIEW_CONTENT')
    ) AS v(role_name, permission)
    ON v.role_name = r.name
)
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT platform_role_id, permission
FROM pairs
WHERE NOT EXISTS (
    SELECT 1 FROM role_permission_platform rpp
    WHERE rpp.platform_role_id = pairs.platform_role_id
      AND rpp.permission = pairs.permission
);
