async fn course_visible_to_learner(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course: &Course,
) -> Result<bool, LearnerCourseCatalogError> {
    if course.lifecycle_status == COURSE_STATUS_PUBLISHED {
        return Ok(true);
    }

    let permission = Permissions::VIEW_COURSE.to_string();
    if user_permission_course_request(conn, actor_user_id, course.id, &permission).await? {
        return Ok(true);
    }

    for organization_id in course_organization_ids(conn, course.id).await? {
        if user_permission_organization_request(conn, actor_user_id, organization_id, &permission)
            .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn user_has_any_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permissions: &[Permissions],
) -> Result<bool, LearnerCourseCatalogError> {
    for permission in permissions {
        if user_has_permission_for_course_context(conn, actor_user_id, course_id, permission)
            .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn user_has_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &Permissions,
) -> Result<bool, LearnerCourseCatalogError> {
    let permission_name = permission.to_string();
    if user_permission_course_request(conn, actor_user_id, course_id, &permission_name).await? {
        return Ok(true);
    }

    if user_permission_platform_request(conn, actor_user_id, &permission_name).await? {
        return Ok(true);
    }

    for organization_id in course_organization_ids(conn, course_id).await? {
        if user_permission_organization_request(
            conn,
            actor_user_id,
            organization_id,
            &permission_name,
        )
        .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn course_organization_ids(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<i32>, LearnerCourseCatalogError> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)
}
