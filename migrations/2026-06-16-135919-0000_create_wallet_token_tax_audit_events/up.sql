CREATE TABLE wallet_token_tax_audit_events (
    id BIGSERIAL PRIMARY KEY,
    actor_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    operation VARCHAR(16) NOT NULL,
    previous_tax_amount NUMERIC NOT NULL,
    new_tax_amount NUMERIC NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT wallet_token_tax_audit_events_operation_check
        CHECK (operation IN ('deposit', 'retire'))
);

CREATE INDEX wallet_token_tax_audit_events_operation_idx
    ON wallet_token_tax_audit_events (operation, created_at DESC, id DESC);

CREATE INDEX wallet_token_tax_audit_events_actor_idx
    ON wallet_token_tax_audit_events (actor_user_id)
    WHERE actor_user_id IS NOT NULL;
