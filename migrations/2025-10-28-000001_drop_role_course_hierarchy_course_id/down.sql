-- Re-add course_id column to role_course_hierarchy
-- Recreates the nullable course_id column and its foreign key to courses(id)

ALTER TABLE role_course_hierarchy
    ADD COLUMN IF NOT EXISTS course_id INT;

-- Add foreign key constraint (explicit name)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'role_course_hierarchy_course_id_fkey'
    ) THEN
        ALTER TABLE role_course_hierarchy
            ADD CONSTRAINT role_course_hierarchy_course_id_fkey
            FOREIGN KEY (course_id) REFERENCES courses(id);
    END IF;
END$$;
