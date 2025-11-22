CREATE TABLE pending_course_organization_invites (
    id SERIAL PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    "order" INTEGER NOT NULL,
    UNIQUE (course_id, organization_id)
);
