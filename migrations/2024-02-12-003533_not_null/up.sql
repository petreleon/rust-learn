-- Your SQL goes here
-- Alter authentications table
ALTER TABLE authentications ALTER COLUMN user_id SET NOT NULL;

-- Alter chapters table
ALTER TABLE chapters ALTER COLUMN course_id SET NOT NULL;

-- Alter contents table
ALTER TABLE contents ALTER COLUMN chapter_id SET NOT NULL;

-- Alter role_organization_hierarchy table
ALTER TABLE role_organization_hierarchy ALTER COLUMN role_id SET NOT NULL, ALTER COLUMN organization_id SET NOT NULL;

-- Alter role_permission_organization table
ALTER TABLE role_permission_organization ALTER COLUMN organization_id SET NOT NULL, ALTER COLUMN role_id SET NOT NULL;

-- Alter role_permission_platform table
ALTER TABLE role_permission_platform ALTER COLUMN role_id SET NOT NULL;

-- Alter role_platform_hierarchy table
ALTER TABLE role_platform_hierarchy ALTER COLUMN role_id SET NOT NULL;

-- Alter user_role_organization table
ALTER TABLE user_role_organization ALTER COLUMN user_id SET NOT NULL, ALTER COLUMN role_id SET NOT NULL, ALTER COLUMN organization_id SET NOT NULL;

-- Alter user_role_platform table
ALTER TABLE user_role_platform ALTER COLUMN user_id SET NOT NULL, ALTER COLUMN role_id SET NOT NULL;
