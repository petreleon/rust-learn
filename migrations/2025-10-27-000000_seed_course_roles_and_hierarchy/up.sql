-- Seed course roles and their default hierarchy (idempotent)
-- Lower hierarchy_level = higher privilege (0 = highest)

-- Insert roles if they do not exist
INSERT INTO course_roles (name, description)
SELECT 'TEACHER', 'Course teacher'
WHERE NOT EXISTS (SELECT 1 FROM course_roles WHERE name = 'TEACHER');

INSERT INTO course_roles (name, description)
SELECT 'STUDENT', 'Course student'
WHERE NOT EXISTS (SELECT 1 FROM course_roles WHERE name = 'STUDENT');

-- Insert default hierarchy rows (course_id NULL means global default)
WITH role_ids AS (
    SELECT id, name FROM course_roles WHERE name IN ('TEACHER','STUDENT')
)
INSERT INTO role_course_hierarchy (course_role_id, course_id, hierarchy_level)
SELECT id,
       NULL::INT AS course_id,
       CASE name
           WHEN 'TEACHER' THEN 0
           WHEN 'STUDENT' THEN 1
       END AS hierarchy_level
FROM role_ids
WHERE NOT EXISTS (
    SELECT 1 FROM role_course_hierarchy rch
    WHERE rch.course_role_id = role_ids.id AND rch.course_id IS NULL
);
