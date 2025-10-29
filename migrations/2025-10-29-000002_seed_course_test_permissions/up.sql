-- Seed course-level test permissions (idempotent)
-- Default (global) assignments using course_id = NULL
--  - STUDENT: TAKE_TESTS
--  - TEACHER: VIEW_TEST_RESULTS

WITH roles AS (
    SELECT id, name FROM course_roles WHERE name IN ('STUDENT','TEACHER')
), pairs AS (
    SELECT r.id AS course_role_id, p.permission
    FROM roles r
    JOIN (
        VALUES
            ('STUDENT','TAKE_TESTS'),
            ('TEACHER','VIEW_TEST_RESULTS')
    ) AS p(role_name, permission)
    ON p.role_name = r.name
)
INSERT INTO role_permission_course (course_id, course_role_id, permission)
SELECT NULL::INT, course_role_id, permission
FROM pairs
WHERE NOT EXISTS (
    SELECT 1 FROM role_permission_course rpc
    WHERE rpc.course_id IS NULL
      AND rpc.course_role_id = pairs.course_role_id
      AND rpc.permission = pairs.permission
);
