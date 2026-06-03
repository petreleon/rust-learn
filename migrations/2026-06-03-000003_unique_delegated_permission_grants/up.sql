WITH duplicate_platform AS (
    SELECT id
    FROM (
        SELECT
            id,
            ROW_NUMBER() OVER (
                PARTITION BY grantee_user_id, permission, COALESCE(expires_at, 'infinity'::timestamptz)
                ORDER BY created_at DESC, id DESC
            ) AS rn
        FROM delegated_permissions
        WHERE revoked_at IS NULL
          AND scope_type = 'platform'
    ) ranked
    WHERE rn > 1
),
duplicate_organization AS (
    SELECT id
    FROM (
        SELECT
            id,
            ROW_NUMBER() OVER (
                PARTITION BY grantee_user_id, permission, organization_id, COALESCE(expires_at, 'infinity'::timestamptz)
                ORDER BY created_at DESC, id DESC
            ) AS rn
        FROM delegated_permissions
        WHERE revoked_at IS NULL
          AND scope_type = 'organization'
    ) ranked
    WHERE rn > 1
),
duplicate_course AS (
    SELECT id
    FROM (
        SELECT
            id,
            ROW_NUMBER() OVER (
                PARTITION BY grantee_user_id, permission, course_id, COALESCE(expires_at, 'infinity'::timestamptz)
                ORDER BY created_at DESC, id DESC
            ) AS rn
        FROM delegated_permissions
        WHERE revoked_at IS NULL
          AND scope_type = 'course'
    ) ranked
    WHERE rn > 1
),
duplicates AS (
    SELECT id FROM duplicate_platform
    UNION ALL
    SELECT id FROM duplicate_organization
    UNION ALL
    SELECT id FROM duplicate_course
)
UPDATE delegated_permissions
SET revoked_at = NOW(),
    updated_at = NOW(),
    revoke_reason = COALESCE(
        revoke_reason,
        'deduplicated duplicate delegated grant before unique retry index'
    )
WHERE id IN (SELECT id FROM duplicates);

CREATE UNIQUE INDEX delegated_permissions_unique_platform_retry_idx
    ON delegated_permissions (
        grantee_user_id,
        permission,
        (COALESCE(expires_at, 'infinity'::timestamptz))
    )
    WHERE revoked_at IS NULL AND scope_type = 'platform';

CREATE UNIQUE INDEX delegated_permissions_unique_organization_retry_idx
    ON delegated_permissions (
        grantee_user_id,
        permission,
        organization_id,
        (COALESCE(expires_at, 'infinity'::timestamptz))
    )
    WHERE revoked_at IS NULL AND scope_type = 'organization';

CREATE UNIQUE INDEX delegated_permissions_unique_course_retry_idx
    ON delegated_permissions (
        grantee_user_id,
        permission,
        course_id,
        (COALESCE(expires_at, 'infinity'::timestamptz))
    )
    WHERE revoked_at IS NULL AND scope_type = 'course';
