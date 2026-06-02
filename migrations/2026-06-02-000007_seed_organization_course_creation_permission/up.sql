WITH roles AS (
    SELECT id, name
    FROM organization_roles
    WHERE name IN ('SUPER_ADMIN', 'ADMIN')
), pairs AS (
    SELECT r.id AS organization_role_id, v.permission
    FROM roles r
    JOIN (
        VALUES
            ('SUPER_ADMIN', 'CREATE_COURSE'),
            ('ADMIN', 'CREATE_COURSE')
    ) AS v(role_name, permission)
    ON v.role_name = r.name
)
INSERT INTO role_permission_organization (organization_id, organization_role_id, permission)
SELECT NULL::INT, organization_role_id, permission
FROM pairs
WHERE NOT EXISTS (
    SELECT 1
    FROM role_permission_organization rpo
    WHERE rpo.organization_id IS NULL
      AND rpo.organization_role_id = pairs.organization_role_id
      AND rpo.permission = pairs.permission
);
