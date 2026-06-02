CREATE UNIQUE INDEX IF NOT EXISTS wallets_unique_user_owner_idx
ON wallets (user_id)
WHERE user_id IS NOT NULL
  AND organization_id IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS wallets_unique_organization_owner_idx
ON wallets (organization_id)
WHERE organization_id IS NOT NULL
  AND user_id IS NULL;
