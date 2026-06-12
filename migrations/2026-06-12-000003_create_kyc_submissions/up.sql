CREATE TABLE kyc_submissions (
    id BIGSERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(32) NOT NULL DEFAULT 'submitted',
    legal_name TEXT NOT NULL,
    country_code VARCHAR(2) NOT NULL,
    document_type VARCHAR(32) NOT NULL,
    document_last4 VARCHAR(16),
    evidence_reference TEXT,
    provider_reference TEXT,
    reviewer_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    rejection_reason TEXT,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT kyc_submissions_status_check CHECK (
        status IN ('submitted', 'under_review', 'verified', 'rejected', 'expired', 'provider_error')
    ),
    CONSTRAINT kyc_submissions_country_code_check CHECK (country_code ~ '^[A-Z]{2}$'),
    CONSTRAINT kyc_submissions_document_last4_check CHECK (
        document_last4 IS NULL OR length(document_last4) BETWEEN 2 AND 16
    )
);

CREATE INDEX kyc_submissions_user_updated_idx
ON kyc_submissions(user_id, updated_at DESC, id DESC);

CREATE INDEX kyc_submissions_review_idx
ON kyc_submissions(status, submitted_at ASC, id ASC);
