CREATE TABLE courses_organizations (
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    "order" INTEGER NOT NULL,
    PRIMARY KEY (course_id, organization_id)
);
