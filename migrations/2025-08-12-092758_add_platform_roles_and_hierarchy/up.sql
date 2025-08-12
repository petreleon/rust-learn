-- Seed platform roles and their hierarchy levels (idempotent)
-- Higher privilege has lower hierarchy_level (0 = highest)

-- Insert roles if they do not exist
INSERT INTO platform_roles (name, description)
SELECT 'SUPER_ADMIN', 'Super administrator'
WHERE NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'SUPER_ADMIN');

INSERT INTO platform_roles (name, description)
SELECT 'ADMIN', 'Administrator'
WHERE NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'ADMIN');

INSERT INTO platform_roles (name, description)
SELECT 'MODERATOR', 'Moderator'
WHERE NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'MODERATOR');

INSERT INTO platform_roles (name, description)
SELECT 'TEACHER', 'Teacher'
WHERE NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'TEACHER');

INSERT INTO platform_roles (name, description)
SELECT 'USER', 'Regular user'
WHERE NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'USER');

INSERT INTO platform_roles (name, description)
SELECT 'STUDENT', 'Student'
WHERE NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'STUDENT');

INSERT INTO platform_roles (name, description)
SELECT 'GUEST', 'Guest'
WHERE NOT EXISTS (SELECT 1 FROM platform_roles WHERE name = 'GUEST');

-- Insert hierarchy rows if missing for each role
WITH role_ids AS (
    SELECT id, name
    FROM platform_roles
    WHERE name IN ('SUPER_ADMIN','ADMIN','MODERATOR','TEACHER','USER','STUDENT','GUEST')
)
INSERT INTO role_platform_hierarchy (platform_role_id, hierarchy_level)
SELECT id,
    CASE name
        WHEN 'SUPER_ADMIN' THEN 0
        WHEN 'ADMIN' THEN 1
        WHEN 'MODERATOR' THEN 2
        WHEN 'TEACHER' THEN 3
        WHEN 'USER' THEN 4
        WHEN 'STUDENT' THEN 5
        WHEN 'GUEST' THEN 6
    END AS hierarchy_level
FROM role_ids
WHERE NOT EXISTS (
    SELECT 1
    FROM role_platform_hierarchy rph
    WHERE rph.platform_role_id = role_ids.id
);
