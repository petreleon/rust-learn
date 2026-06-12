WITH pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN', 'REVIEW_KYC_SUBMISSIONS'),
        ('ADMIN', 'REVIEW_KYC_SUBMISSIONS')
)
DELETE FROM role_permission_platform rpp
USING platform_roles pr, pairs
WHERE rpp.platform_role_id = pr.id
  AND pr.name = pairs.role_name
  AND rpp.permission = pairs.permission;
