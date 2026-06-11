async fn delegated_teacher_course_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let now = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
        .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_COURSE))
        .filter(delegated_permissions::organization_id.is_null())
        .filter(delegated_permissions::permission.eq_any(permission_names))
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select(delegated_permissions::course_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}

async fn delegated_teacher_organization_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let now = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
        .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_ORGANIZATION))
        .filter(delegated_permissions::course_id.is_null())
        .filter(delegated_permissions::permission.eq_any(permission_names))
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select(delegated_permissions::organization_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}

async fn courses_for_organizations(
    conn: &mut AsyncPgConnection,
    organization_ids: &[i32],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    if organization_ids.is_empty() {
        return Ok(Vec::new());
    }

    courses_organizations::table
        .filter(courses_organizations::organization_id.eq_any(organization_ids))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)
}

async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, OrganizationCourseListError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission_name)
        .await
        .map_err(OrganizationCourseListError::from)?
    {
        return Ok(true);
    }

    user_permission_organization_request(conn, actor_user_id, organization_id, &permission_name)
        .await
        .map_err(OrganizationCourseListError::from)
}
