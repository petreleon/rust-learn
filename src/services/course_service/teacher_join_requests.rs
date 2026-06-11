async fn load_teacher_course_roster_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRosterSummary, TeacherCourseDashboardError> {
    let enrolled_student_count = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("STUDENT"))
        .count()
        .get_result::<i64>(conn)
        .await?;
    let pending_join_request_count =
        count_course_join_requests_by_status(conn, course_id, COURSE_JOIN_STATUS_PENDING).await?;
    let waitlisted_join_request_count =
        count_course_join_requests_by_status(conn, course_id, COURSE_JOIN_STATUS_WAITLISTED)
            .await?;

    Ok(TeacherCourseRosterSummary {
        enrolled_student_count,
        pending_join_request_count,
        waitlisted_join_request_count,
    })
}

async fn count_course_join_requests_by_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    status: &str,
) -> Result<i64, TeacherCourseDashboardError> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::status.eq(status))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)
}

async fn load_teacher_course_join_request_page(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    query: &TeacherCourseEnrollmentQuery,
    can_manage_enrollments: bool,
) -> Result<TeacherCourseJoinRequestPage, TeacherCourseDashboardError> {
    let mut count_query = course_join_requests::table.into_boxed();
    count_query = count_query.filter(course_join_requests::course_id.eq(course_id));
    count_query = apply_join_request_status_filter(count_query, query.status.as_deref());
    let total = count_query.count().get_result::<i64>(conn).await?;

    let mut list_query = course_join_requests::table.into_boxed();
    list_query = list_query.filter(course_join_requests::course_id.eq(course_id));
    list_query = apply_join_request_status_filter(list_query, query.status.as_deref());
    let rows = list_query
        .order(course_join_requests::updated_at.desc())
        .then_order_by(course_join_requests::id.asc())
        .limit(query.limit)
        .offset(query.offset)
        .load::<CourseJoinRequest>(conn)
        .await?;

    let mut requests = Vec::with_capacity(rows.len());
    for request in rows {
        let requester = load_teacher_enrollment_user_summary(conn, request.requester_user_id)
            .await?
            .ok_or_else(|| {
                TeacherCourseDashboardError::Database(format!(
                    "missing requester user for join request {}",
                    request.id
                ))
            })?;
        let reviewer = match request.reviewer_user_id {
            Some(user_id) => load_teacher_enrollment_user_summary(conn, user_id).await?,
            None => None,
        };
        let can_decide = can_manage_enrollments
            && matches!(
                request.status.as_str(),
                COURSE_JOIN_STATUS_PENDING | COURSE_JOIN_STATUS_WAITLISTED
            );

        requests.push(TeacherCourseJoinRequestItem {
            id: request.id,
            status: request.status,
            requester,
            reviewer,
            decision_reason: request.decision_reason,
            created_at: request.created_at,
            updated_at: request.updated_at,
            decided_at: request.decided_at,
            can_decide,
        });
    }

    Ok(TeacherCourseJoinRequestPage {
        requests,
        total,
        limit: query.limit,
        offset: query.offset,
        status: query.status.clone(),
    })
}
