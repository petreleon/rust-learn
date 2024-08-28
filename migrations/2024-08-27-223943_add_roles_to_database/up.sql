-- Your SQL goes here

INSERT INTO roles (name, description) VALUES
('SUPER_ADMIN', 'Super admin with all permissions'),
('ADMIN', 'Admin with administrative permissions'),
('MODERATOR', 'Moderator with permissions to manage content and users'),
('USER', 'Regular user with limited permissions'),
('GUEST', 'Guest user with minimal permissions'),
('TEACHER', 'Teacher with permissions to manage courses and students'),
('TEACHER_COURSE', 'Teacher with permissions limited to specific courses'),
('STUDENT', 'Student with permissions to access courses and content'),
('STUDENT_COURSE', 'Student with permissions limited to specific courses');
