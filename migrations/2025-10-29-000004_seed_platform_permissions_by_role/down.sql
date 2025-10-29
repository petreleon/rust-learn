-- Rollback curated platform permissions per role (ADMIN, MODERATOR, TEACHER, USER, STUDENT, GUEST)
WITH pairs(role_name, permission) AS (
    VALUES
        -- ADMIN
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
        ('ADMIN','MANAGE_WALLETS'),
        ('ADMIN','MANAGE_PAYMENT_METHODS'),
        ('ADMIN','VIEW_FINANCIAL_REPORTS'),
        ('ADMIN','VIEW_TRANSACTIONS'),
        ('ADMIN','APPROVE_CENTRALIZED_TRANSFER'),
        ('ADMIN','RECONCILE_WALLETS'),
        ('ADMIN','RUN_TESTS'),
        ('ADMIN','VIEW_TEST_RESULTS'),
        ('ADMIN','MANAGE_TEST_SUITES'),
        -- MODERATOR
        ('MODERATOR','MANAGE_DISCUSSIONS'),
        ('MODERATOR','MODERATE_DISCUSSION'),
        ('MODERATOR','POST_IN_DISCUSSION'),
        ('MODERATOR','VIEW_NOTIFICATION'),
        ('MODERATOR','SEND_NOTIFICATION'),
        ('MODERATOR','VIEW_REPORT'),
        ('MODERATOR','VIEW_USER'),
        ('MODERATOR','VIEW_TEST_RESULTS'),
        -- TEACHER
        ('TEACHER','POST_IN_DISCUSSION'),
        ('TEACHER','SEND_NOTIFICATION'),
        ('TEACHER','VIEW_NOTIFICATION'),
        ('TEACHER','VIEW_REPORT'),
        ('TEACHER','GENERATE_REPORT'),
        ('TEACHER','RUN_TESTS'),
        ('TEACHER','VIEW_TEST_RESULTS'),
        -- USER
        ('USER','VIEW_NOTIFICATION'),
        ('USER','POST_IN_DISCUSSION'),
        ('USER','VIEW_COURSE'),
        ('USER','VIEW_CONTENT'),
        ('USER','REQUEST_JOIN_COURSE'),
        ('USER','REQUEST_JOIN_ORGANIZATION'),
        ('USER','JOIN_COURSE'),
        -- STUDENT
        ('STUDENT','VIEW_NOTIFICATION'),
        ('STUDENT','POST_IN_DISCUSSION'),
        ('STUDENT','VIEW_COURSE'),
        ('STUDENT','VIEW_CONTENT'),
        ('STUDENT','REQUEST_JOIN_COURSE'),
        ('STUDENT','JOIN_COURSE'),
        -- GUEST
        ('GUEST','VIEW_NOTIFICATION'),
        ('GUEST','VIEW_COURSE'),
        ('GUEST','VIEW_CONTENT')
)
DELETE FROM role_permission_platform rpp
USING platform_roles pr
WHERE rpp.platform_role_id = pr.id
  AND pr.name IN ('ADMIN','MODERATOR','TEACHER','USER','STUDENT','GUEST')
  AND rpp.permission IN (SELECT permission FROM pairs WHERE role_name = pr.name);
