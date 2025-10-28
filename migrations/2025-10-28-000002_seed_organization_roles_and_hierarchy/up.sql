-- Seed organization roles and their default hierarchy (idempotent)
-- Lower hierarchy_level = higher privilege (0 = highest)

-- Roles to add: DIRECTOR, DEPUTY_DIRECTOR, TEACHER, STUDENT, ACCOUNTANT

-- Insert roles if they do not exist
INSERT INTO organization_roles (name, description)
SELECT 'DIRECTOR', 'Organization director'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'DIRECTOR');

INSERT INTO organization_roles (name, description)
SELECT 'DEPUTY_DIRECTOR', 'Deputy director'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'DEPUTY_DIRECTOR');

INSERT INTO organization_roles (name, description)
SELECT 'TEACHER', 'Organization teacher'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'TEACHER');

INSERT INTO organization_roles (name, description)
SELECT 'STUDENT', 'Organization student'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'STUDENT');

INSERT INTO organization_roles (name, description)
SELECT 'ACCOUNTANT', 'Organization accountant'
WHERE NOT EXISTS (SELECT 1 FROM organization_roles WHERE name = 'ACCOUNTANT');

-- Insert default global hierarchy rows (column organization_id was removed in a prior migration)
WITH role_ids AS (
    SELECT id, name FROM organization_roles WHERE name IN ('DIRECTOR','DEPUTY_DIRECTOR','TEACHER','STUDENT','ACCOUNTANT')
)
INSERT INTO role_organization_hierarchy (organization_role_id, hierarchy_level)
SELECT id,
       CASE name
           WHEN 'DIRECTOR' THEN 0
           WHEN 'DEPUTY_DIRECTOR' THEN 1
           WHEN 'ACCOUNTANT' THEN 2
           WHEN 'TEACHER' THEN 3
           WHEN 'STUDENT' THEN 4
       END AS hierarchy_level
FROM role_ids
WHERE NOT EXISTS (
    SELECT 1 FROM role_organization_hierarchy rch
    WHERE rch.organization_role_id = role_ids.id
);
