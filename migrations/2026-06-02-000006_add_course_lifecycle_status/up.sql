ALTER TABLE courses
ADD COLUMN lifecycle_status VARCHAR(32) NOT NULL DEFAULT 'draft';

ALTER TABLE courses
ADD CONSTRAINT courses_lifecycle_status_check
CHECK (
    lifecycle_status IN (
        'draft',
        'submitted',
        'needs_changes',
        'approved',
        'published',
        'archived',
        'suspended'
    )
);

CREATE INDEX courses_lifecycle_status_idx
    ON courses (lifecycle_status);
