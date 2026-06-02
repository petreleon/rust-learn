CREATE TABLE reward_execution_jobs (
    id BIGSERIAL PRIMARY KEY,
    reward_candidate_id BIGINT NOT NULL REFERENCES reward_candidates(id) ON DELETE CASCADE,
    status VARCHAR(32) NOT NULL DEFAULT 'queued',
    attempts INTEGER NOT NULL DEFAULT 0,
    run_after TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reward_execution_jobs_candidate_unique
        UNIQUE (reward_candidate_id),
    CONSTRAINT reward_execution_jobs_status_check
        CHECK (status IN ('queued', 'in_progress', 'succeeded', 'failed')),
    CONSTRAINT reward_execution_jobs_attempts_check
        CHECK (attempts >= 0)
);

CREATE INDEX reward_execution_jobs_status_run_after_idx
    ON reward_execution_jobs (status, run_after);
