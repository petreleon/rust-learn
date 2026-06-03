DROP INDEX IF EXISTS teacher_applications_idempotency_key_idx;

ALTER TABLE teacher_applications
    DROP COLUMN IF EXISTS idempotency_key;
