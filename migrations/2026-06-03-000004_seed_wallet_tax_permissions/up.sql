-- Seed platform permissions for configuring token taxes charged when the
-- platform pays Ethereum gas for wallet deposit or retire operations.

WITH pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN', 'SET_DEPOSIT_TAX'),
        ('SUPER_ADMIN', 'SET_RETIRE_TAX'),
        ('ADMIN', 'SET_DEPOSIT_TAX'),
        ('ADMIN', 'SET_RETIRE_TAX')
)
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT pr.id, pairs.permission
FROM pairs
JOIN platform_roles pr ON pr.name = pairs.role_name
WHERE NOT EXISTS (
    SELECT 1
    FROM role_permission_platform rpp
    WHERE rpp.platform_role_id = pr.id
      AND rpp.permission = pairs.permission
);
