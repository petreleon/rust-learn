pub async fn remove_course_enrollment(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    target_user_id: i32,
) -> Result<CourseEnrollmentRemovalResponse, CourseEnrollmentError> {
    ensure_course_exists(conn, course_id).await?;
    ensure_enrollment_management_permission(conn, actor_user_id, course_id).await?;

    let student_role_id = CourseRole::find_by_name("STUDENT", conn).await?;
    let removed_count = diesel::delete(
        user_role_course::table
            .filter(user_role_course::user_id.eq(Some(target_user_id)))
            .filter(user_role_course::course_id.eq(Some(course_id)))
            .filter(user_role_course::course_role_id.eq(Some(student_role_id))),
    )
    .execute(conn)
    .await?;

    if removed_count == 0 {
        return Err(CourseEnrollmentError::NotFound);
    }

    Ok(CourseEnrollmentRemovalResponse {
        course_id,
        user_id: target_user_id,
        removed: true,
    })
}

async fn ensure_course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    courses::table
        .find(course_id)
        .select(courses::id)
        .first::<i32>(conn)
        .await?;
    Ok(())
}

async fn ensure_enrollment_management_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    let permission = Permissions::MANAGE_COURSE_ENROLLMENTS.to_string();
    if user_has_permission_for_course_context(conn, user_id, course_id, &permission).await? {
        return Ok(());
    }

    Err(CourseEnrollmentError::PermissionDenied(permission))
}

async fn ensure_join_request_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    for permission in [
        Permissions::REQUEST_JOIN_COURSE.to_string(),
        Permissions::JOIN_COURSE.to_string(),
    ] {
        if user_has_permission_for_course_context(conn, user_id, course_id, &permission).await? {
            return Ok(());
        }
    }

    Err(CourseEnrollmentError::PermissionDenied(
        Permissions::REQUEST_JOIN_COURSE.to_string(),
    ))
}

async fn ensure_join_approval_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    for permission in [
        Permissions::APPROVE_COURSE_JOIN_REQUESTS.to_string(),
        Permissions::MANAGE_COURSE_ENROLLMENTS.to_string(),
    ] {
        if user_has_permission_for_course_context(conn, user_id, course_id, &permission).await? {
            return Ok(());
        }
    }

    Err(CourseEnrollmentError::PermissionDenied(
        Permissions::APPROVE_COURSE_JOIN_REQUESTS.to_string(),
    ))
}

async fn user_has_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> Result<bool, CourseEnrollmentError> {
    if user_permission_course_request(conn, user_id, course_id, permission).await? {
        return Ok(true);
    }

    if user_permission_platform_request(conn, user_id, permission).await? {
        return Ok(true);
    }

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;

    for organization_id in organization_ids {
        if user_permission_organization_request(conn, user_id, organization_id, permission).await? {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn has_course_student_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<bool, CourseEnrollmentError> {
    let student_role_id = CourseRole::find_by_name("STUDENT", conn).await?;
    diesel::select(exists(
        user_role_course::table
            .filter(user_role_course::user_id.eq(user_id))
            .filter(user_role_course::course_id.eq(course_id))
            .filter(user_role_course::course_role_id.eq(student_role_id)),
    ))
    .get_result(conn)
    .await
    .map_err(CourseEnrollmentError::from)
}

async fn assign_student_course_role_if_missing(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    if has_course_student_role(conn, user_id, course_id).await? {
        return Ok(());
    }

    let student_role_id = CourseRole::find_by_name("STUDENT", conn).await?;
    UserRoleCourse::assign(conn, user_id, course_id, student_role_id).await?;
    Ok(())
}
