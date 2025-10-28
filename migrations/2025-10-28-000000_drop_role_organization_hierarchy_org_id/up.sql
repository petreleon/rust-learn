-- Drop organization_id column from role_organization_hierarchy
-- This removes per-organization scoping on the hierarchy and makes hierarchy global

ALTER TABLE role_organization_hierarchy
    DROP COLUMN IF EXISTS organization_id CASCADE;
