-- Revert canonical organization role names back to the legacy form
-- This will rename SUPER_ADMIN back to SUPERADMIN if present (idempotent)

UPDATE organization_roles
SET name = 'SUPERADMIN'
WHERE name = 'SUPER_ADMIN' AND NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'SUPER_ADMIN');

-- Note: We do not delete roles here to avoid accidental data loss.

