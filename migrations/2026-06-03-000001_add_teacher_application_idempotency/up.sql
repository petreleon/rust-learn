ALTER TABLE teacher_applications
    ADD COLUMN idempotency_key TEXT;

CREATE UNIQUE INDEX teacher_applications_idempotency_key_idx
    ON teacher_applications (idempotency_key)
    WHERE idempotency_key IS NOT NULL;
