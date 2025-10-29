-- Rollback course-level test permissions seeding
DELETE FROM role_permission_course rpc
USING course_roles cr
WHERE rpc.course_role_id = cr.id
  AND rpc.course_id IS NULL
  AND cr.name IN ('STUDENT','TEACHER')
  AND rpc.permission IN ('TAKE_TESTS','VIEW_TEST_RESULTS');
