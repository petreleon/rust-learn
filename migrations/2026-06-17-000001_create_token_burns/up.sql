CREATE TABLE token_burn_requests (
    id BIGSERIAL PRIMARY KEY,
    actor_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    burner_type VARCHAR(32) NOT NULL,
    user_id INTEGER REFERENCES users(id) ON DELETE RESTRICT,
    organization_id INTEGER REFERENCES organizations(id) ON DELETE RESTRICT,
    wallet_id INTEGER REFERENCES wallets(id) ON DELETE SET NULL,
    source VARCHAR(48) NOT NULL,
    fee_path VARCHAR(48) NOT NULL,
    status VARCHAR(48) NOT NULL,
    amount NUMERIC NOT NULL,
    fee_amount NUMERIC NOT NULL DEFAULT 0,
    idempotency_key VARCHAR(128) NOT NULL,
    deposit_intent_id BIGINT REFERENCES wallet_token_deposit_intents(id) ON DELETE SET NULL,
    transaction_id BIGINT REFERENCES transactions(id) ON DELETE SET NULL,
    external_transaction_id BIGINT REFERENCES external_transactions(id) ON DELETE SET NULL,
    internal_transaction_id BIGINT REFERENCES internal_transactions(id) ON DELETE SET NULL,
    permission_evidence VARCHAR(128),
    wallet_provider VARCHAR(32) NOT NULL DEFAULT 'metamask',
    metamask_required BOOLEAN NOT NULL DEFAULT false,
    wallet_action VARCHAR(64) NOT NULL,
    leaderboard_visible BOOLEAN NOT NULL DEFAULT true,
    last_error TEXT,
    confirmed_at TIMESTAMPTZ,
    ledger_recorded_at TIMESTAMPTZ,
    leaderboard_indexed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT token_burn_requests_burner_type_check
        CHECK (burner_type IN ('user', 'organization')),
    CONSTRAINT token_burn_requests_source_check
        CHECK (source IN (
            'centralized_wallet',
            'decentralized_direct',
            'decentralized_platform_mediated'
        )),
    CONSTRAINT token_burn_requests_fee_path_check
        CHECK (fee_path IN (
            'network_fee_paid_by_user',
            'platform_deposit_fee',
            'platform_subsidized',
            'none'
        )),
    CONSTRAINT token_burn_requests_status_check
        CHECK (status IN (
            'requested',
            'approval_pending',
            'deposit_pending',
            'burn_pending',
            'confirmed',
            'ledger_recorded',
            'leaderboard_indexed',
            'failed',
            'needs_reconciliation'
        )),
    CONSTRAINT token_burn_requests_amount_check CHECK (amount > 0),
    CONSTRAINT token_burn_requests_fee_amount_check CHECK (fee_amount >= 0),
    CONSTRAINT token_burn_requests_burner_owner_check CHECK (
        (burner_type = 'user' AND user_id IS NOT NULL AND organization_id IS NULL)
        OR
        (burner_type = 'organization' AND organization_id IS NOT NULL AND user_id IS NULL)
    )
);

CREATE UNIQUE INDEX token_burn_requests_idempotency_key_idx
    ON token_burn_requests (idempotency_key);

CREATE INDEX token_burn_requests_user_idx
    ON token_burn_requests (user_id, created_at DESC, id DESC);

CREATE INDEX token_burn_requests_organization_idx
    ON token_burn_requests (organization_id, created_at DESC, id DESC);

CREATE TABLE token_burn_fee_records (
    id BIGSERIAL PRIMARY KEY,
    burn_request_id BIGINT NOT NULL REFERENCES token_burn_requests(id) ON DELETE CASCADE,
    actor_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    fee_path VARCHAR(48) NOT NULL,
    amount NUMERIC NOT NULL,
    transaction_id BIGINT REFERENCES transactions(id) ON DELETE SET NULL,
    deposit_intent_id BIGINT REFERENCES wallet_token_deposit_intents(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT token_burn_fee_records_fee_path_check
        CHECK (fee_path IN (
            'network_fee_paid_by_user',
            'platform_deposit_fee',
            'platform_subsidized',
            'none'
        )),
    CONSTRAINT token_burn_fee_records_amount_check CHECK (amount >= 0)
);

CREATE INDEX token_burn_fee_records_burn_request_idx
    ON token_burn_fee_records (burn_request_id);

CREATE TABLE token_burn_leaderboard_events (
    id BIGSERIAL PRIMARY KEY,
    burn_request_id BIGINT NOT NULL REFERENCES token_burn_requests(id) ON DELETE CASCADE,
    burner_type VARCHAR(32) NOT NULL,
    user_id INTEGER REFERENCES users(id) ON DELETE RESTRICT,
    organization_id INTEGER REFERENCES organizations(id) ON DELETE RESTRICT,
    amount NUMERIC NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    visible BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT token_burn_leaderboard_events_burner_type_check
        CHECK (burner_type IN ('user', 'organization')),
    CONSTRAINT token_burn_leaderboard_events_amount_check CHECK (amount > 0),
    CONSTRAINT token_burn_leaderboard_events_burner_owner_check CHECK (
        (burner_type = 'user' AND user_id IS NOT NULL AND organization_id IS NULL)
        OR
        (burner_type = 'organization' AND organization_id IS NOT NULL AND user_id IS NULL)
    )
);

CREATE UNIQUE INDEX token_burn_leaderboard_events_request_idx
    ON token_burn_leaderboard_events (burn_request_id);

CREATE INDEX token_burn_leaderboard_events_window_idx
    ON token_burn_leaderboard_events (visible, occurred_at DESC, id DESC);

CREATE INDEX token_burn_leaderboard_events_user_idx
    ON token_burn_leaderboard_events (user_id, occurred_at DESC)
    WHERE user_id IS NOT NULL;

CREATE INDEX token_burn_leaderboard_events_organization_idx
    ON token_burn_leaderboard_events (organization_id, occurred_at DESC)
    WHERE organization_id IS NOT NULL;

WITH platform_pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN', 'VIEW_BURN_LEADERBOARD'),
        ('SUPER_ADMIN', 'RECONCILE_TOKEN_BURNS'),
        ('ADMIN', 'VIEW_BURN_LEADERBOARD'),
        ('ADMIN', 'RECONCILE_TOKEN_BURNS')
)
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT pr.id, platform_pairs.permission
FROM platform_pairs
JOIN platform_roles pr ON pr.name = platform_pairs.role_name
WHERE NOT EXISTS (
    SELECT 1
    FROM role_permission_platform rpp
    WHERE rpp.platform_role_id = pr.id
      AND rpp.permission = platform_pairs.permission
);

WITH organization_pairs(role_name, permission) AS (
    VALUES
        ('SUPER_ADMIN', 'BURN_ORGANIZATION_TOKENS'),
        ('ADMIN', 'BURN_ORGANIZATION_TOKENS'),
        ('SUPER_ADMIN', 'VIEW_BURN_LEADERBOARD'),
        ('ADMIN', 'VIEW_BURN_LEADERBOARD')
)
INSERT INTO role_permission_organization (organization_id, organization_role_id, permission)
SELECT NULL::INT, organization_roles.id, organization_pairs.permission
FROM organization_pairs
JOIN organization_roles ON organization_roles.name = organization_pairs.role_name
WHERE NOT EXISTS (
    SELECT 1
    FROM role_permission_organization rpo
    WHERE rpo.organization_id IS NULL
      AND rpo.organization_role_id = organization_roles.id
      AND rpo.permission = organization_pairs.permission
);
