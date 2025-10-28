-- Drop course_id column from role_course_hierarchy
-- This removes per-course scoping on the hierarchy and makes hierarchy global

ALTER TABLE role_course_hierarchy
    DROP COLUMN IF EXISTS course_id CASCADE;
