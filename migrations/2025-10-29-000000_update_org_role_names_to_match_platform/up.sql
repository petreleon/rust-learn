-- Migration: 2025-10-29_update_org_role_names_to_match_platform
-- Purpose: Normalize organization role names to match the platform naming
-- convention. Several variants of the "super admin" role (for example
-- with spaces, dashes or no separator) are updated to the single canonical
-- name 'SUPER_ADMIN'. If the role has no description, a sensible default
-- will be set.
--
-- Behaviour:
--  - Change the `name` field to 'SUPER_ADMIN' for matching rows.
--  - Set `description` to 'Organization super administrator' only when
--    the existing value is NULL (uses COALESCE to preserve any existing
--    descriptions).
--
-- Rollback: This is a non-reversible data migration (information about the
-- original exact spelling/casing is lost once renamed). To roll back you
-- would need to map 'SUPER_ADMIN' back to the previous variants explicitly
-- (e.g. with an UPDATE that targets the newly-named rows and sets them
-- back to their former values). Run such a rollback only if you have a
-- reliable record of the original names to restore.
--
-- Created: 2025-10-29

UPDATE organization_roles
SET name = 'SUPER_ADMIN', description = COALESCE(description, 'Organization super administrator')
WHERE name IN ('SUPERADMIN', 'SUPER ADMIN', 'SUPER-ADMIN');

