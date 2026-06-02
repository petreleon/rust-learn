DROP INDEX IF EXISTS courses_lifecycle_status_idx;

ALTER TABLE courses
DROP CONSTRAINT IF EXISTS courses_lifecycle_status_check;

ALTER TABLE courses
DROP COLUMN IF EXISTS lifecycle_status;
