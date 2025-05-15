-- Rename role_id columns to match association expectations

ALTER TABLE user_role_platform
    RENAME COLUMN role_id TO platform_role_id;

ALTER TABLE user_role_organization
    RENAME COLUMN role_id TO organization_role_id;

ALTER TABLE user_role_course
    RENAME COLUMN role_id TO course_role_id;

ALTER TABLE role_platform_hierarchy
    RENAME COLUMN role_id TO platform_role_id;

ALTER TABLE role_organization_hierarchy
    RENAME COLUMN role_id TO organization_role_id;

ALTER TABLE role_course_hierarchy
    RENAME COLUMN role_id TO course_role_id;

ALTER TABLE role_permission_platform
    RENAME COLUMN role_id TO platform_role_id;

ALTER TABLE role_permission_organization
    RENAME COLUMN role_id TO organization_role_id;

ALTER TABLE role_permission_course
    RENAME COLUMN role_id TO course_role_id;
