-- This file should undo anything in `up.sql`
-- Remove seeded hierarchy and roles (safe: only if names match)

-- Delete hierarchy rows for the seeded roles
DELETE FROM role_platform_hierarchy
WHERE platform_role_id IN (
    SELECT id FROM platform_roles
    WHERE name IN ('SUPER_ADMIN','ADMIN','MODERATOR','TEACHER','USER','STUDENT','GUEST')
);

-- Delete the seeded roles themselves
DELETE FROM platform_roles
WHERE name IN ('SUPER_ADMIN','ADMIN','MODERATOR','TEACHER','USER','STUDENT','GUEST');
