-- Reverse migration for wallets

-- Reverse migration for minimal wallets migration
DROP INDEX IF EXISTS wallets_user_id_idx;
DROP INDEX IF EXISTS wallets_organization_id_idx;
DROP TABLE IF EXISTS wallets;

DROP TABLE IF EXISTS wallets;
