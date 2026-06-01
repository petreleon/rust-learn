-- Align SUPER_ADMIN storage permission with Permissions::MANAGE_S3_OBJECTS.
-- The earlier seed used the legacy MANAGE_MINIO_OBJECTS name.
INSERT INTO role_permission_platform (platform_role_id, permission)
SELECT pr.id, 'MANAGE_S3_OBJECTS'
FROM platform_roles pr
WHERE pr.name = 'SUPER_ADMIN'
  AND NOT EXISTS (
      SELECT 1
      FROM role_permission_platform rpp
      WHERE rpp.platform_role_id = pr.id
        AND rpp.permission = 'MANAGE_S3_OBJECTS'
  );

DELETE FROM role_permission_platform rpp
USING platform_roles pr
WHERE rpp.platform_role_id = pr.id
  AND pr.name = 'SUPER_ADMIN'
  AND rpp.permission = 'MANAGE_MINIO_OBJECTS';
