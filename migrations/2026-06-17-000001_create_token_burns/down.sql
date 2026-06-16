DELETE FROM role_permission_organization
WHERE organization_id IS NULL
  AND permission IN ('BURN_ORGANIZATION_TOKENS', 'VIEW_BURN_LEADERBOARD')
  AND organization_role_id IN (
      SELECT id
      FROM organization_roles
      WHERE name IN ('SUPER_ADMIN', 'ADMIN')
  );

DELETE FROM role_permission_platform
WHERE permission IN ('VIEW_BURN_LEADERBOARD', 'RECONCILE_TOKEN_BURNS')
  AND platform_role_id IN (
      SELECT id
      FROM platform_roles
      WHERE name IN ('SUPER_ADMIN', 'ADMIN')
  );

DROP TABLE IF EXISTS token_burn_leaderboard_events;
DROP TABLE IF EXISTS token_burn_fee_records;
DROP TABLE IF EXISTS token_burn_requests;
