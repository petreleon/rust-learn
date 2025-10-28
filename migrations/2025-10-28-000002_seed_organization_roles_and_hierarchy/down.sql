-- Roll back seeded organization roles and hierarchy
-- Remove hierarchy entries and roles
DELETE FROM role_organization_hierarchy
WHERE organization_role_id IN (
    SELECT id FROM organization_roles WHERE name IN ('SUPERADMIN','ADMIN','TEACHER','STUDENT','MODERATOR')
);

-- Remove roles themselves
DELETE FROM organization_roles WHERE name IN ('SUPERADMIN','ADMIN','TEACHER','STUDENT','MODERATOR');
