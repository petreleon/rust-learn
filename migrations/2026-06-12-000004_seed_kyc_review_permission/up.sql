WITH pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN', 'REVIEW_KYC_SUBMISSIONS'),
        ('ADMIN', 'REVIEW_KYC_SUBMISSIONS')
)
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT pr.id, pairs.permission
FROM platform_roles pr
JOIN pairs ON pairs.role_name = pr.name
WHERE NOT EXISTS (
    SELECT 1 FROM role_permission_platform rpp
    WHERE rpp.platform_role_id = pr.id
      AND rpp.permission = pairs.permission
);
