-- Remove only the business workflow permissions introduced by this migration.

WITH platform_pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN','SUBMIT_TEACHER_APPLICATION'),
        ('SUPER_ADMIN','REVIEW_TEACHER_APPLICATIONS'),
        ('SUPER_ADMIN','APPROVE_TEACHER_APPLICATION'),
        ('SUPER_ADMIN','REJECT_TEACHER_APPLICATION'),
        ('SUPER_ADMIN','DELEGATE_REWARD_APPROVAL'),
        ('SUPER_ADMIN','SET_REWARD_POLICY'),
        ('SUPER_ADMIN','APPROVE_REWARD_AMOUNT'),
        ('SUPER_ADMIN','EXECUTE_REWARD_PAYOUT'),
        ('SUPER_ADMIN','VIEW_REWARD_AUDIT'),
        ('SUPER_ADMIN','MANAGE_REWARD_FRAUD_BLOCKS'),
        ('SUPER_ADMIN','BLOCK_REWARD_TEACHER'),
        ('SUPER_ADMIN','BLOCK_REWARD_ORGANIZATION'),
        ('SUPER_ADMIN','NOMINATE_TEACHER_FOR_PLATFORM_REVIEW'),
        ('SUPER_ADMIN','VIEW_ORG_TEACHER_APPLICATIONS'),
        ('SUPER_ADMIN','SUBMIT_ORG_COURSE_REWARD_EVENT'),
        ('SUPER_ADMIN','VIEW_ORG_REWARD_REPORTS'),
        ('SUPER_ADMIN','MANAGE_ORG_REWARD_BUDGET'),
        ('SUPER_ADMIN','CREATE_REWARDABLE_COURSE_EVENT'),
        ('SUPER_ADMIN','VIEW_COURSE_REWARD_STATUS'),
        ('SUPER_ADMIN','SUBMIT_COURSE_REWARD_EVENT'),
        ('SUPER_ADMIN','APPROVE_STUDENT_REWARD_CANDIDATE'),
        ('SUPER_ADMIN','GRADE_REWARDABLE_ASSESSMENT'),
        ('SUPER_ADMIN','MANAGE_COURSE_REWARD_RULES'),
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
        ('MODERATOR','APPROVE_REWARD_AMOUNT'),
        ('MODERATOR','VIEW_REWARD_AUDIT'),
        ('TEACHER','SUBMIT_TEACHER_APPLICATION'),
        ('USER','SUBMIT_TEACHER_APPLICATION'),
        ('STUDENT','SUBMIT_TEACHER_APPLICATION')
)
DELETE FROM role_permission_platform rpp
USING platform_roles pr, platform_pairs
WHERE rpp.platform_role_id = pr.id
  AND pr.name = platform_pairs.role_name
  AND rpp.permission = platform_pairs.permission;

WITH organization_pairs(role_name, permission) AS (
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
)
DELETE FROM role_permission_organization rpo
USING organization_roles org_role, organization_pairs
WHERE rpo.organization_id IS NULL
  AND rpo.organization_role_id = org_role.id
  AND org_role.name = organization_pairs.role_name
  AND rpo.permission = organization_pairs.permission;

WITH course_pairs(role_name, permission) AS (
    VALUES
        ('TEACHER','CREATE_REWARDABLE_COURSE_EVENT'),
        ('TEACHER','VIEW_COURSE_REWARD_STATUS'),
        ('TEACHER','SUBMIT_COURSE_REWARD_EVENT'),
        ('TEACHER','APPROVE_STUDENT_REWARD_CANDIDATE'),
        ('TEACHER','GRADE_REWARDABLE_ASSESSMENT'),
        ('TEACHER','MANAGE_COURSE_REWARD_RULES'),
        ('STUDENT','VIEW_COURSE_REWARD_STATUS')
)
DELETE FROM role_permission_course rpc
USING course_roles cr, course_pairs
WHERE rpc.course_id IS NULL
  AND rpc.course_role_id = cr.id
  AND cr.name = course_pairs.role_name
  AND rpc.permission = course_pairs.permission;
