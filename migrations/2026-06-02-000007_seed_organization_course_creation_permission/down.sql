WITH pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN', 'CREATE_COURSE'),
        ('ADMIN', 'CREATE_COURSE')
)
DELETE FROM role_permission_organization rpo
USING organization_roles org_role, pairs
WHERE rpo.organization_id IS NULL
  AND rpo.organization_role_id = org_role.id
  AND org_role.name = pairs.role_name
  AND rpo.permission = pairs.permission;
