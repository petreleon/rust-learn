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

async fn assign_approved_teaching_bundle(
    conn: &mut AsyncPgConnection,
    application: &TeacherApplication,
) -> diesel::QueryResult<()> {
    match application.requested_scope.as_str() {
        TEACHER_APPLICATION_SCOPE_PLATFORM => {
            let role_id = PlatformRole::find_by_name(&Roles::TEACHER.to_string(), conn).await?;
            assign_platform_role_if_missing(conn, application.applicant_user_id, role_id).await
        }
        TEACHER_APPLICATION_SCOPE_ORGANIZATION => {
            let organization_id = application
                .requested_organization_id
                .or(application.organization_sponsor_id)
                .ok_or(diesel::result::Error::NotFound)?;
            let role_id = OrganizationRole::find_by_name("TEACHER", conn).await?;
            assign_organization_role_if_missing(
                conn,
                application.applicant_user_id,
                organization_id,
                role_id,
            )
            .await
        }
        TEACHER_APPLICATION_SCOPE_COURSE => {
            let course_id = application
                .requested_course_id
                .ok_or(diesel::result::Error::NotFound)?;
            let role_id = CourseRole::find_by_name("TEACHER", conn).await?;
            assign_course_role_if_missing(conn, application.applicant_user_id, course_id, role_id)
                .await
        }
        _ => Err(diesel::result::Error::NotFound),
    }
}

async fn assign_platform_role_if_missing(
    conn: &mut AsyncPgConnection,
    target_user_id: i32,
    platform_role_id: i32,
) -> diesel::QueryResult<()> {
    let already_assigned = diesel::select(diesel::dsl::exists(
        user_role_platform::table
            .filter(user_role_platform::user_id.eq(target_user_id))
            .filter(user_role_platform::platform_role_id.eq(platform_role_id)),
    ))
    .get_result::<bool>(conn)
    .await?;

    if !already_assigned {
        UserRolePlatform::assign(conn, target_user_id, platform_role_id).await?;
    }
    Ok(())
}

async fn assign_organization_role_if_missing(
    conn: &mut AsyncPgConnection,
    target_user_id: i32,
    organization_id: i32,
    organization_role_id: i32,
) -> diesel::QueryResult<()> {
    let already_assigned = diesel::select(diesel::dsl::exists(
        user_role_organization::table
            .filter(user_role_organization::user_id.eq(Some(target_user_id)))
            .filter(user_role_organization::organization_id.eq(Some(organization_id)))
            .filter(user_role_organization::organization_role_id.eq(Some(organization_role_id))),
    ))
    .get_result::<bool>(conn)
    .await?;

    if !already_assigned {
        UserRoleOrganization::assign(conn, target_user_id, organization_id, organization_role_id)
            .await?;
    }
    Ok(())
}

async fn assign_course_role_if_missing(
    conn: &mut AsyncPgConnection,
    target_user_id: i32,
    course_id: i32,
    course_role_id: i32,
) -> diesel::QueryResult<()> {
    let already_assigned = diesel::select(diesel::dsl::exists(
        user_role_course::table
            .filter(user_role_course::user_id.eq(Some(target_user_id)))
            .filter(user_role_course::course_id.eq(Some(course_id)))
            .filter(user_role_course::course_role_id.eq(Some(course_role_id))),
    ))
    .get_result::<bool>(conn)
    .await?;

    if !already_assigned {
        UserRoleCourse::assign(conn, target_user_id, course_id, course_role_id).await?;
    }
    Ok(())
}

async fn ensure_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: Permissions,
) -> Result<(), TeacherApplicationError> {
    let permission_name = permission.to_string();
    match user_permission_platform_request(conn, user_id, &permission_name).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(TeacherApplicationError::PermissionDenied(permission_name)),
        Err(error) => Err(TeacherApplicationError::from(error)),
    }
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
