-- Revert renamed columns back to role_id

ALTER TABLE user_role_platform
    RENAME COLUMN platform_role_id TO role_id;

ALTER TABLE user_role_organization
    RENAME COLUMN organization_role_id TO role_id;

ALTER TABLE user_role_course
    RENAME COLUMN course_role_id TO role_id;

ALTER TABLE role_platform_hierarchy
    RENAME COLUMN platform_role_id TO role_id;

ALTER TABLE role_organization_hierarchy
    RENAME COLUMN organization_role_id TO role_id;

ALTER TABLE role_course_hierarchy
    RENAME COLUMN course_role_id TO role_id;

ALTER TABLE role_permission_platform
    RENAME COLUMN platform_role_id TO role_id;

ALTER TABLE role_permission_organization
    RENAME COLUMN organization_role_id TO role_id;

ALTER TABLE role_permission_course
    RENAME COLUMN course_role_id TO role_id;
