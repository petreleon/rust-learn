-- Rollback course-level permissions for TEACHER and STUDENT seeded by this migration
WITH pairs(role_name, permission) AS (
    VALUES
        -- TEACHER
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
        -- STUDENT
        ('STUDENT','VIEW_COURSE'),
        ('STUDENT','VIEW_CONTENT'),
        ('STUDENT','TAKE_TESTS'),
        ('STUDENT','VIEW_ASSESSMENT'),
        ('STUDENT','POST_IN_DISCUSSION'),
        ('STUDENT','REQUEST_JOIN_COURSE'),
        ('STUDENT','JOIN_COURSE'),
        ('STUDENT','VIEW_NOTIFICATION')
)
DELETE FROM role_permission_course rpc
USING course_roles cr
WHERE rpc.course_role_id = cr.id
  AND rpc.course_id IS NULL
  AND cr.name IN ('TEACHER','STUDENT')
  AND rpc.permission IN (SELECT permission FROM pairs WHERE role_name = cr.name);
