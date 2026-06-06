WITH pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN', 'SET_DEPOSIT_TAX'),
        ('SUPER_ADMIN', 'SET_RETIRE_TAX'),
        ('ADMIN', 'SET_DEPOSIT_TAX'),
        ('ADMIN', 'SET_RETIRE_TAX')
)
DELETE FROM role_permission_platform rpp
USING platform_roles pr, pairs
WHERE rpp.platform_role_id = pr.id
  AND pr.name = pairs.role_name
  AND rpp.permission = pairs.permission;
