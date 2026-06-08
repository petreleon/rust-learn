-- This file should undo anything in `up.sql`
-- Revert authentications table
ALTER TABLE authentications ALTER COLUMN user_id DROP NOT NULL;

-- Revert chapters table
ALTER TABLE chapters ALTER COLUMN course_id DROP NOT NULL;

-- Revert contents table
ALTER TABLE contents ALTER COLUMN chapter_id DROP NOT NULL;

-- Revert role_organization_hierarchy table
ALTER TABLE role_organization_hierarchy ALTER COLUMN role_id DROP NOT NULL, ALTER COLUMN organization_id DROP NOT NULL;

-- Revert role_permission_organization table
ALTER TABLE role_permission_organization ALTER COLUMN organization_id DROP NOT NULL, ALTER COLUMN role_id DROP NOT NULL;

-- Revert role_permission_platform table
ALTER TABLE role_permission_platform ALTER COLUMN role_id DROP NOT NULL;

-- Revert role_platform_hierarchy table
ALTER TABLE role_platform_hierarchy ALTER COLUMN role_id DROP NOT NULL;

-- Revert user_role_organization table
ALTER TABLE user_role_organization ALTER COLUMN user_id DROP NOT NULL, ALTER COLUMN role_id DROP NOT NULL, ALTER COLUMN organization_id DROP NOT NULL;

-- Revert user_role_platform table
ALTER TABLE user_role_platform ALTER COLUMN user_id DROP NOT NULL, ALTER COLUMN role_id DROP NOT NULL;
