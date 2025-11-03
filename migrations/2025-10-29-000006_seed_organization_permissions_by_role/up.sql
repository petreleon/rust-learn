-- Seed curated organization-level permissions per role (idempotent)
-- Roles covered: SUPER_ADMIN, ADMIN, MODERATOR, TEACHER, STUDENT
-- Scope: organization (uses role_permission_organization with organization_id = NULL for global defaults)

WITH roles AS (
    SELECT id, name FROM organization_roles
    WHERE name IN ('SUPER_ADMIN','ADMIN','MODERATOR','TEACHER','STUDENT')
), pairs AS (
    -- Map each organization role to a curated set of organization-scoped permissions
    SELECT r.id AS organization_role_id, v.permission
    FROM roles r
    JOIN (
        -- SUPER_ADMIN (organization scope): full control of org settings, members, billing, wallets, comms, discussions, and reporting
        VALUES
            ('SUPER_ADMIN','MANAGE_ORG_MEMBERS'),
            ('SUPER_ADMIN','MANAGE_ORG_SETTINGS'),
            ('SUPER_ADMIN','MANAGE_ORG_BILLING'),
            ('SUPER_ADMIN','MANAGE_ORG_WALLETS'),
            ('SUPER_ADMIN','INVITE_USER_TO_ORGANIZATION'),
            ('SUPER_ADMIN','VIEW_USER'),
            ('SUPER_ADMIN','VIEW_ORGANIZATION'),
            ('SUPER_ADMIN','VIEW_REPORT'),
            ('SUPER_ADMIN','GENERATE_REPORT'),
            ('SUPER_ADMIN','MANAGE_NOTIFICATION_TEMPLATES'),
            ('SUPER_ADMIN','SEND_BULK_NOTIFICATION'),
            ('SUPER_ADMIN','SEND_NOTIFICATION'),
            ('SUPER_ADMIN','VIEW_NOTIFICATION'),
            ('SUPER_ADMIN','MANAGE_DISCUSSIONS'),
            ('SUPER_ADMIN','MODERATE_DISCUSSION'),
            ('SUPER_ADMIN','POST_IN_DISCUSSION'),

        -- ADMIN (organization scope): manage members, settings, billing, wallets, comms, discussions, and reporting
            ('ADMIN','MANAGE_ORG_MEMBERS'),
            ('ADMIN','MANAGE_ORG_SETTINGS'),
            ('ADMIN','MANAGE_ORG_BILLING'),
            ('ADMIN','MANAGE_ORG_WALLETS'),
            ('ADMIN','INVITE_USER_TO_ORGANIZATION'),
            ('ADMIN','VIEW_USER'),
            ('ADMIN','VIEW_ORGANIZATION'),
            ('ADMIN','VIEW_REPORT'),
            ('ADMIN','GENERATE_REPORT'),
            ('ADMIN','SEND_NOTIFICATION'),
            ('ADMIN','VIEW_NOTIFICATION'),
            ('ADMIN','MANAGE_DISCUSSIONS'),
            ('ADMIN','MODERATE_DISCUSSION'),
            ('ADMIN','POST_IN_DISCUSSION'),

        -- MODERATOR (organization scope): discussions, limited comms and visibility
            ('MODERATOR','MANAGE_DISCUSSIONS'),
            ('MODERATOR','MODERATE_DISCUSSION'),
            ('MODERATOR','POST_IN_DISCUSSION'),
            ('MODERATOR','VIEW_NOTIFICATION'),
            ('MODERATOR','SEND_NOTIFICATION'),
            ('MODERATOR','VIEW_USER'),

        -- TEACHER (organization scope): announcements and reporting visibility
            ('TEACHER','POST_IN_DISCUSSION'),
            ('TEACHER','SEND_NOTIFICATION'),
            ('TEACHER','VIEW_NOTIFICATION'),
            ('TEACHER','VIEW_REPORT'),
            ('TEACHER','GENERATE_REPORT'),

        -- STUDENT (organization scope): minimal visibility and participation
            ('STUDENT','VIEW_NOTIFICATION'),
            ('STUDENT','POST_IN_DISCUSSION'),
            ('STUDENT','VIEW_ORGANIZATION')
    ) AS v(role_name, permission)
    ON v.role_name = r.name
)
INSERT INTO role_permission_organization (organization_id, organization_role_id, permission)
SELECT NULL::INT, organization_role_id, permission
FROM pairs
WHERE NOT EXISTS (
    SELECT 1 FROM role_permission_organization rpo
    WHERE rpo.organization_id IS NULL
      AND rpo.organization_role_id = pairs.organization_role_id
      AND rpo.permission = pairs.permission
);
