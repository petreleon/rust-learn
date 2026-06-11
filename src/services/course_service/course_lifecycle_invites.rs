pub async fn update_course_lifecycle(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    request: CourseLifecycleUpdateRequest,
) -> Result<Course, CourseLifecycleError> {
    let target_status = normalize_course_status(&request.status)?;
    ensure_lifecycle_permission(conn, actor_user_id, course_id, &target_status).await?;

    diesel::update(courses::table.find(course_id))
        .set(courses::lifecycle_status.eq(target_status))
        .get_result::<Course>(conn)
        .await
        .map_err(CourseLifecycleError::from)
}

async fn ensure_lifecycle_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    target_status: &str,
) -> Result<(), CourseLifecycleError> {
    let required_course_permission = match target_status {
        COURSE_STATUS_DRAFT
        | COURSE_STATUS_SUBMITTED
        | COURSE_STATUS_ARCHIVED
        | COURSE_STATUS_SUSPENDED => Permissions::MANAGE_COURSE_SETTINGS,
        COURSE_STATUS_NEEDS_CHANGES | COURSE_STATUS_APPROVED => Permissions::APPROVE_COURSE_CONTENT,
        COURSE_STATUS_PUBLISHED => Permissions::PUBLISH_CONTENT,
        _ => {
            return Err(CourseLifecycleError::InvalidStatus(
                "unsupported course lifecycle status".to_string(),
            ))
        }
    };

    let course_permission_name = required_course_permission.to_string();
    if user_permission_course_request(conn, user_id, course_id, &course_permission_name).await? {
        return Ok(());
    }

    let platform_permission_name = Permissions::MODIFY_COURSE.to_string();
    if user_permission_platform_request(conn, user_id, &platform_permission_name).await? {
        return Ok(());
    }

    Err(CourseLifecycleError::PermissionDenied(
        course_permission_name,
    ))
}

fn normalize_course_status(status: &str) -> Result<String, CourseLifecycleError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        COURSE_STATUS_DRAFT
        | COURSE_STATUS_SUBMITTED
        | COURSE_STATUS_NEEDS_CHANGES
        | COURSE_STATUS_APPROVED
        | COURSE_STATUS_PUBLISHED
        | COURSE_STATUS_ARCHIVED
        | COURSE_STATUS_SUSPENDED => Ok(normalized),
        _ => Err(CourseLifecycleError::InvalidStatus(
            "unsupported course lifecycle status".to_string(),
        )),
    }
}

pub async fn create_course_organization_invite(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) -> QueryResult<usize> {
    let max_order_active: Option<i32> = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(diesel::dsl::max(courses_organizations::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let max_order_pending: Option<i32> = pending_course_organization_invites::table
        .filter(pending_course_organization_invites::course_id.eq(course_id))
        .select(diesel::dsl::max(pending_course_organization_invites::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let next_order = match (max_order_active, max_order_pending) {
        (Some(a), Some(b)) => std::cmp::max(a, b) + 1,
        (Some(a), None) => a + 1,
        (None, Some(b)) => b + 1,
        (None, None) => 0,
    };

    let new_invite = NewPendingCourseOrganizationInvite {
        course_id,
        organization_id,
        order: next_order,
    };
    diesel::insert_into(pending_course_organization_invites::table)
        .values(&new_invite)
        .execute(conn)
        .await
}

pub async fn accept_course_organization_invite(
    conn: &mut AsyncPgConnection,
    invite_id: i32,
) -> QueryResult<usize> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let invite = pending_course_organization_invites::table
                .find(invite_id)
                .first::<PendingCourseOrganizationInvite>(conn)
                .await?;

            let new_link = NewCourseOrganization {
                course_id: invite.course_id,
                organization_id: invite.organization_id,
                order: invite.order,
            };

            diesel::insert_into(courses_organizations::table)
                .values(&new_link)
                .execute(conn)
                .await?;

            diesel::delete(pending_course_organization_invites::table.find(invite_id))
                .execute(conn)
                .await
        })
    })
    .await
}
