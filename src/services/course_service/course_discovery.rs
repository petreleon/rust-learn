pub async fn discover_courses(
    conn: &mut AsyncPgConnection,
    discovery: CourseDiscoveryQuery,
) -> QueryResult<CourseDiscoveryResponse> {
    let mut count_query = courses::table.into_boxed();
    let mut list_query = courses::table.into_boxed();

    if let Some(search) = discovery.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        count_query = count_query.filter(
            courses::title
                .ilike(pattern.clone())
                .escape(LIKE_ESCAPE_CHAR),
        );
        list_query = list_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(organization_id) = discovery.organization_id {
        let course_ids_for_organization = || {
            courses_organizations::table
                .filter(courses_organizations::organization_id.eq(organization_id))
                .select(courses_organizations::course_id)
        };

        count_query = count_query.filter(courses::id.eq_any(course_ids_for_organization()));
        list_query = list_query.filter(courses::id.eq_any(course_ids_for_organization()));
    }

    let total = count_query.count().get_result(conn).await?;
    let courses = list_query
        .order(courses::id.asc())
        .limit(discovery.limit)
        .offset(discovery.offset)
        .load::<Course>(conn)
        .await?;

    Ok(CourseDiscoveryResponse {
        courses,
        total,
        limit: discovery.limit,
        offset: discovery.offset,
        search: discovery.search,
        organization_id: discovery.organization_id,
    })
}

pub async fn discover_teacher_course_dashboard(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    dashboard_query: TeacherCourseDashboardQuery,
) -> Result<TeacherCourseDashboardResponse, TeacherCourseDashboardError> {
    let candidate_scope = teacher_course_candidate_scope(conn, actor_user_id).await?;
    if matches!(
        candidate_scope,
        TeacherCourseCandidateScope::CourseIds(ref ids) if ids.is_empty()
    ) {
        return Ok(TeacherCourseDashboardResponse {
            courses: Vec::new(),
            total: 0,
            limit: dashboard_query.limit,
            offset: dashboard_query.offset,
            search: dashboard_query.search,
            lifecycle_status: dashboard_query.lifecycle_status,
        });
    }

    let mut count_query = courses::table.into_boxed();
    let mut list_query = courses::table.into_boxed();

    match &candidate_scope {
        TeacherCourseCandidateScope::All => {}
        TeacherCourseCandidateScope::CourseIds(course_ids) => {
            count_query = count_query.filter(courses::id.eq_any(course_ids));
            list_query = list_query.filter(courses::id.eq_any(course_ids));
        }
    }

    if let Some(search) = dashboard_query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        count_query = count_query.filter(
            courses::title
                .ilike(pattern.clone())
                .escape(LIKE_ESCAPE_CHAR),
        );
        list_query = list_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(status) = dashboard_query.lifecycle_status.as_deref() {
        count_query = count_query.filter(courses::lifecycle_status.eq(status));
        list_query = list_query.filter(courses::lifecycle_status.eq(status));
    }

    let total = count_query
        .count()
        .get_result(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let candidate_courses = list_query
        .order(courses::id.asc())
        .limit(dashboard_query.limit)
        .offset(dashboard_query.offset)
        .load::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;

    let mut items = Vec::new();
    for course in candidate_courses {
        let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
        if !permissions.has_teacher_access() {
            continue;
        }

        items.push(build_teacher_course_dashboard_item(conn, course, permissions).await?);
    }

    Ok(TeacherCourseDashboardResponse {
        courses: items,
        total,
        limit: dashboard_query.limit,
        offset: dashboard_query.offset,
        search: dashboard_query.search,
        lifecycle_status: dashboard_query.lifecycle_status,
    })
}
