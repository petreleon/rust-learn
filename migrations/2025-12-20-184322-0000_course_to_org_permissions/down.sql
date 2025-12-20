-- Revert changes

-- Remove new permissions from COURSE TEACHER
DELETE FROM role_permission_course
WHERE course_role_id = (SELECT id FROM course_roles WHERE name = 'TEACHER')
AND permission IN ('APPROVE_COURSE_JOIN_REQUESTS', 'ADD_STUDENT_FROM_ORGANIZATION');

-- Restore to COURSE role
INSERT INTO role_permission_course (course_role_id, permission)
SELECT id, 'JOIN_COURSE'
FROM course_roles
WHERE name = 'STUDENT';

INSERT INTO role_permission_course (course_role_id, permission)
SELECT id, 'REQUEST_JOIN_COURSE'
FROM course_roles
WHERE name = 'STUDENT';

-- Restore to PLATFORM role (If they were there, usually they were)
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT id, 'JOIN_COURSE'
FROM platform_roles
WHERE name = 'STUDENT';

INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT id, 'REQUEST_JOIN_COURSE'
FROM platform_roles
WHERE name = 'STUDENT';

-- Remove permissions from ORGANIZATION STUDENT
DELETE FROM role_permission_organization
WHERE organization_role_id = (SELECT id FROM organization_roles WHERE name = 'STUDENT')
AND permission IN ('JOIN_COURSE', 'REQUEST_JOIN_COURSE');
