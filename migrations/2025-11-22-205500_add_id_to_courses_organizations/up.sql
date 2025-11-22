ALTER TABLE courses_organizations DROP CONSTRAINT courses_organizations_pkey;
ALTER TABLE courses_organizations ADD COLUMN id SERIAL PRIMARY KEY;
ALTER TABLE courses_organizations ADD CONSTRAINT courses_organizations_course_id_organization_id_key UNIQUE (course_id, organization_id);
