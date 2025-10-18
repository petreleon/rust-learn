ALTER TABLE organization_roles ADD COLUMN created_for INTEGER REFERENCES users(id);
ALTER TABLE course_roles ADD COLUMN created_for INTEGER REFERENCES users(id);