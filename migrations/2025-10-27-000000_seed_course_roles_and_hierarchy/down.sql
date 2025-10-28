-- Roll back seeded course roles and hierarchy
-- Remove hierarchy entries for default (NULL course_id)
DELETE FROM role_course_hierarchy
WHERE course_id IS NULL
  AND course_role_id IN (
      SELECT id FROM course_roles WHERE name IN ('TEACHER','STUDENT')
  );

-- Remove roles themselves
DELETE FROM course_roles WHERE name IN ('TEACHER','STUDENT');
