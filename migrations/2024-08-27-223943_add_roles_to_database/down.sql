-- This file should undo anything in `up.sql`

DELETE FROM roles WHERE name IN (
    'SUPER_ADMIN',
    'ADMIN',
    'MODERATOR',
    'USER',
    'GUEST',
    'TEACHER',
    'TEACHER_COURSE',
    'STUDENT',
    'STUDENT_COURSE'
);
