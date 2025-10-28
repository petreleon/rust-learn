
-- Roles to add: SUPERADMIN, ADMIN, TEACHER, STUDENT, MODERATOR

INSERT INTO organization_roles (name, description)
SELECT 'SUPERADMIN', 'Organization super administrator'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'SUPERADMIN');

INSERT INTO organization_roles (name, description)
SELECT 'ADMIN', 'Organization administrator'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'ADMIN');

INSERT INTO organization_roles (name, description)
SELECT 'TEACHER', 'Organization teacher'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'TEACHER');

INSERT INTO organization_roles (name, description)
SELECT 'STUDENT', 'Organization student'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'STUDENT');

INSERT INTO organization_roles (name, description)
SELECT 'MODERATOR', 'Organization moderator'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'MODERATOR');

WITH role_ids AS (
    SELECT id, name FROM organization_roles WHERE name IN ('SUPERADMIN','ADMIN','TEACHER','STUDENT','MODERATOR')
)
INSERT INTO role_organization_hierarchy (organization_role_id, hierarchy_level)
SELECT id,
       CASE name
           WHEN 'SUPERADMIN' THEN 0
           WHEN 'ADMIN' THEN 1
           WHEN 'MODERATOR' THEN 2
           WHEN 'TEACHER' THEN 3
           WHEN 'STUDENT' THEN 4
       END AS hierarchy_level
FROM role_ids
WHERE NOT EXISTS (
    SELECT 1 FROM role_organization_hierarchy rch
    WHERE rch.organization_role_id = role_ids.id
);
