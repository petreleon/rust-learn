-- Move JOIN_COURSE and REQUEST_JOIN_COURSE from COURSE STUDENT to ORGANIZATION STUDENT

INSERT INTO role_permission_organization (organization_role_id, permission)
SELECT id, 'JOIN_COURSE'
FROM organization_roles
WHERE name = 'STUDENT';

INSERT INTO role_permission_organization (organization_role_id, permission)
SELECT id, 'REQUEST_JOIN_COURSE'
FROM organization_roles
WHERE name = 'STUDENT';

-- Remove from COURSE role (The request source)
DELETE FROM role_permission_course
WHERE course_role_id = (SELECT id FROM course_roles WHERE name = 'STUDENT')
AND permission IN ('JOIN_COURSE', 'REQUEST_JOIN_COURSE');

-- Remove from PLATFORM role (To be clean/consistent)
DELETE FROM role_permission_platform
WHERE platform_role_id = (SELECT id FROM platform_roles WHERE name = 'STUDENT')
AND permission IN ('JOIN_COURSE', 'REQUEST_JOIN_COURSE');

-- Add new permissions to COURSE TEACHER
INSERT INTO role_permission_course (course_role_id, permission)
SELECT id, 'APPROVE_COURSE_JOIN_REQUESTS'
FROM course_roles
WHERE name = 'TEACHER';

INSERT INTO role_permission_course (course_role_id, permission)
SELECT id, 'ADD_STUDENT_FROM_ORGANIZATION'
FROM course_roles
WHERE name = 'TEACHER';
