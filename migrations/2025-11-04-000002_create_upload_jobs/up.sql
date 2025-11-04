-- Create upload_jobs table for background video processing jobs
CREATE TABLE IF NOT EXISTS upload_jobs (
    id BIGSERIAL PRIMARY KEY,
    bucket VARCHAR NOT NULL,
    object TEXT NOT NULL,
    user_id INT NULL REFERENCES users(id) ON DELETE SET NULL,
    status VARCHAR NOT NULL DEFAULT 'queued',
    attempts INT NOT NULL DEFAULT 0,
    last_error TEXT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE INDEX IF NOT EXISTS idx_upload_jobs_status ON upload_jobs(status);
CREATE INDEX IF NOT EXISTS idx_upload_jobs_user_id ON upload_jobs(user_id);
