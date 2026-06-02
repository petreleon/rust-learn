-- Seed business workflow permissions for teacher applications, rewards, and fraud controls.
-- Business authorization must check scoped permissions, not role names.

WITH perms(permission) AS (
    VALUES
        ('SUBMIT_TEACHER_APPLICATION'),
        ('REVIEW_TEACHER_APPLICATIONS'),
        ('APPROVE_TEACHER_APPLICATION'),
        ('REJECT_TEACHER_APPLICATION'),
        ('DELEGATE_REWARD_APPROVAL'),
        ('SET_REWARD_POLICY'),
        ('APPROVE_REWARD_AMOUNT'),
        ('EXECUTE_REWARD_PAYOUT'),
        ('VIEW_REWARD_AUDIT'),
        ('MANAGE_REWARD_FRAUD_BLOCKS'),
        ('BLOCK_REWARD_TEACHER'),
        ('BLOCK_REWARD_ORGANIZATION'),
        ('NOMINATE_TEACHER_FOR_PLATFORM_REVIEW'),
        ('VIEW_ORG_TEACHER_APPLICATIONS'),
        ('SUBMIT_ORG_COURSE_REWARD_EVENT'),
        ('VIEW_ORG_REWARD_REPORTS'),
        ('MANAGE_ORG_REWARD_BUDGET'),
        ('CREATE_REWARDABLE_COURSE_EVENT'),
        ('VIEW_COURSE_REWARD_STATUS'),
        ('SUBMIT_COURSE_REWARD_EVENT'),
        ('APPROVE_STUDENT_REWARD_CANDIDATE'),
        ('GRADE_REWARDABLE_ASSESSMENT'),
        ('MANAGE_COURSE_REWARD_RULES')
), super_admin AS (
    SELECT id FROM platform_roles WHERE name = 'SUPER_ADMIN'
)
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT sa.id, p.permission
FROM perms p CROSS JOIN super_admin sa
WHERE NOT EXISTS (
    SELECT 1 FROM role_permission_platform rpp
    WHERE rpp.platform_role_id = sa.id
      AND rpp.permission = p.permission
);

WITH roles AS (
    SELECT id, name FROM platform_roles
    WHERE name IN ('ADMIN','MODERATOR','TEACHER','USER','STUDENT')
), pairs AS (
    SELECT r.id AS platform_role_id, v.permission
    FROM roles r
    JOIN (
        VALUES
            -- Platform administration reviews teacher applications, sets policies,
            -- approves payout amounts, executes payouts, and handles fraud blocks.
            ('ADMIN','REVIEW_TEACHER_APPLICATIONS'),
            ('ADMIN','APPROVE_TEACHER_APPLICATION'),
            ('ADMIN','REJECT_TEACHER_APPLICATION'),
            ('ADMIN','DELEGATE_REWARD_APPROVAL'),
            ('ADMIN','SET_REWARD_POLICY'),
            ('ADMIN','APPROVE_REWARD_AMOUNT'),
            ('ADMIN','EXECUTE_REWARD_PAYOUT'),
            ('ADMIN','VIEW_REWARD_AUDIT'),
            ('ADMIN','MANAGE_REWARD_FRAUD_BLOCKS'),
            ('ADMIN','BLOCK_REWARD_TEACHER'),
            ('ADMIN','BLOCK_REWARD_ORGANIZATION'),

            -- Platform moderators can review payout amounts and audits by default.
            -- They do not receive candidate approval or fraud-block permissions here.
            ('MODERATOR','APPROVE_REWARD_AMOUNT'),
            ('MODERATOR','VIEW_REWARD_AUDIT'),

            -- Authenticated participant bundles can apply for teacher review.
            ('TEACHER','SUBMIT_TEACHER_APPLICATION'),
            ('USER','SUBMIT_TEACHER_APPLICATION'),
            ('STUDENT','SUBMIT_TEACHER_APPLICATION')
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

WITH roles AS (
    SELECT id, name FROM organization_roles
    WHERE name IN ('SUPER_ADMIN','ADMIN')
), pairs AS (
    SELECT r.id AS organization_role_id, v.permission
    FROM roles r
    JOIN (
        VALUES
            ('SUPER_ADMIN','NOMINATE_TEACHER_FOR_PLATFORM_REVIEW'),
            ('SUPER_ADMIN','VIEW_ORG_TEACHER_APPLICATIONS'),
            ('SUPER_ADMIN','SUBMIT_ORG_COURSE_REWARD_EVENT'),
            ('SUPER_ADMIN','VIEW_ORG_REWARD_REPORTS'),
            ('SUPER_ADMIN','MANAGE_ORG_REWARD_BUDGET'),
            ('ADMIN','NOMINATE_TEACHER_FOR_PLATFORM_REVIEW'),
            ('ADMIN','VIEW_ORG_TEACHER_APPLICATIONS'),
            ('ADMIN','SUBMIT_ORG_COURSE_REWARD_EVENT'),
            ('ADMIN','VIEW_ORG_REWARD_REPORTS'),
            ('ADMIN','MANAGE_ORG_REWARD_BUDGET')
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

WITH roles AS (
    SELECT id, name FROM course_roles WHERE name IN ('TEACHER','STUDENT')
), pairs AS (
    SELECT r.id AS course_role_id, v.permission
    FROM roles r
    JOIN (
        VALUES
            ('TEACHER','CREATE_REWARDABLE_COURSE_EVENT'),
            ('TEACHER','VIEW_COURSE_REWARD_STATUS'),
            ('TEACHER','SUBMIT_COURSE_REWARD_EVENT'),
            ('TEACHER','APPROVE_STUDENT_REWARD_CANDIDATE'),
            ('TEACHER','GRADE_REWARDABLE_ASSESSMENT'),
            ('TEACHER','MANAGE_COURSE_REWARD_RULES'),
            ('STUDENT','VIEW_COURSE_REWARD_STATUS')
    ) AS v(role_name, permission)
    ON v.role_name = r.name
)
INSERT INTO role_permission_course (course_id, course_role_id, permission)
SELECT NULL::INT, course_role_id, permission
FROM pairs
WHERE NOT EXISTS (
    SELECT 1 FROM role_permission_course rpc
    WHERE rpc.course_id IS NULL
      AND rpc.course_role_id = pairs.course_role_id
      AND rpc.permission = pairs.permission
);
