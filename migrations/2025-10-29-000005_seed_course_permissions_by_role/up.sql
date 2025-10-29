-- Seed comprehensive course-level permissions for TEACHER and STUDENT (idempotent)
-- Default (global) assignments using course_id = NULL
-- TEACHER gets course management, content/assessment control, discussions, and test visibility/runs
-- STUDENT gets participation and assessment/test-taking permissions

WITH roles AS (
    SELECT id, name FROM course_roles WHERE name IN ('TEACHER','STUDENT')
), pairs AS (
    SELECT r.id AS course_role_id, v.permission
    FROM roles r
    JOIN (
        -- TEACHER permissions at course scope
        VALUES
            ('TEACHER','VIEW_COURSE'),
            ('TEACHER','MANAGE_COURSE_SETTINGS'),
            ('TEACHER','MANAGE_COURSE_ENROLLMENTS'),
            ('TEACHER','APPROVE_COURSE_CONTENT'),
            ('TEACHER','PUBLISH_CONTENT'),
            ('TEACHER','CREATE_CONTENT'),
            ('TEACHER','MODIFY_CONTENT'),
            ('TEACHER','DELETE_CONTENT'),
            ('TEACHER','VIEW_CONTENT'),
            ('TEACHER','CREATE_ASSESSMENT'),
            ('TEACHER','MODIFY_ASSESSMENT'),
            ('TEACHER','DELETE_ASSESSMENT'),
            ('TEACHER','GRADE_ASSESSMENT'),
            ('TEACHER','VIEW_ASSESSMENT'),
            ('TEACHER','RUN_TESTS'),
            ('TEACHER','VIEW_TEST_RESULTS'),
            ('TEACHER','POST_IN_DISCUSSION'),
            ('TEACHER','MODERATE_DISCUSSION'),
            ('TEACHER','SEND_NOTIFICATION'),
            ('TEACHER','VIEW_NOTIFICATION'),

        -- STUDENT permissions at course scope
            ('STUDENT','VIEW_COURSE'),
            ('STUDENT','VIEW_CONTENT'),
            ('STUDENT','TAKE_TESTS'),
            ('STUDENT','VIEW_ASSESSMENT'),
            ('STUDENT','POST_IN_DISCUSSION'),
            ('STUDENT','REQUEST_JOIN_COURSE'),
            ('STUDENT','JOIN_COURSE'),
            ('STUDENT','VIEW_NOTIFICATION')
    ) AS v(role_name, permission)
    ON v.role_name = r.name
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
