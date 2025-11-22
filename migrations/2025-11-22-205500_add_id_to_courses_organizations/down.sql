ALTER TABLE courses_organizations DROP CONSTRAINT courses_organizations_course_id_organization_id_key;
ALTER TABLE courses_organizations DROP COLUMN id;
ALTER TABLE courses_organizations ADD PRIMARY KEY (course_id, organization_id);
