async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, TeacherApplicationError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, user_id, &permission_name).await? {
        return Ok(true);
    }
    Ok(
        user_permission_organization_request(conn, user_id, organization_id, &permission_name)
            .await?,
    )
}

async fn ensure_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<(), TeacherApplicationError> {
    let permission_name = permission.to_string();
    match user_permission_organization_request(conn, user_id, organization_id, &permission_name)
        .await
    {
        Ok(true) => Ok(()),
        Ok(false) => Err(TeacherApplicationError::PermissionDenied(permission_name)),
        Err(error) => Err(TeacherApplicationError::from(error)),
    }
}
