async fn build_learner_course_catalog_item(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course: Course,
) -> Result<LearnerCourseCatalogItem, LearnerCourseCatalogError> {
    let organizations = load_learner_course_organizations(conn, course.id).await?;
    let teachers = load_learner_course_teachers(conn, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let rewards = load_learner_course_reward_summary(conn, course.id).await?;
    let access = build_learner_course_access(conn, actor_user_id, course.id).await?;
    let enrollment = build_learner_course_enrollment(
        conn,
        actor_user_id,
        course.id,
        &course.lifecycle_status,
        access.can_request_join,
    )
    .await?;

    Ok(LearnerCourseCatalogItem {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        description: course.description.clone(),
        topics: course
            .topics
            .clone()
            .and_then(|t| serde_json::from_str::<Vec<String>>(&t).ok())
            .unwrap_or_default(),
        prerequisites: course
            .prerequisites
            .clone()
            .map(|p| vec![p])
            .unwrap_or_default(),
        organizations,
        teachers,
        content,
        rewards,
        enrollment,
        access,
    })
}

async fn build_organization_course_list_item(
    conn: &mut AsyncPgConnection,
    course: Course,
    permissions: OrganizationCoursePermissionSummary,
) -> Result<OrganizationCourseListItem, OrganizationCourseListError> {
    let teachers = load_learner_course_teachers(conn, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let rewards = load_learner_course_reward_summary(conn, course.id).await?;
    let roster = load_teacher_course_roster_summary(conn, course.id).await?;
    let reward_queue = load_teacher_course_reward_queue_summary(conn, course.id).await?;

    Ok(OrganizationCourseListItem {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        teachers,
        content,
        rewards,
        roster,
        reward_queue,
        permissions,
    })
}

async fn build_teacher_course_dashboard_item(
    conn: &mut AsyncPgConnection,
    course: Course,
    permissions: TeacherCoursePermissionSummary,
) -> Result<TeacherCourseDashboardItem, TeacherCourseDashboardError> {
    let organizations = load_learner_course_organizations(conn, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let rewards = load_learner_course_reward_summary(conn, course.id).await?;
    let roster = load_teacher_course_roster_summary(conn, course.id).await?;
    let reward_queue = load_teacher_course_reward_queue_summary(conn, course.id).await?;

    Ok(TeacherCourseDashboardItem {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        organizations,
        content,
        rewards,
        roster,
        reward_queue,
        permissions,
    })
}

async fn load_teacher_course_workspace_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    course_lifecycle_status: &str,
) -> Result<Vec<TeacherCourseWorkspaceChapter>, TeacherCourseDashboardError> {
    let chapter_rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .then_order_by(chapters::id.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await?;

    let mut result = Vec::with_capacity(chapter_rows.len());
    for (id, title, order) in chapter_rows {
        let content_rows = contents::table
            .filter(contents::chapter_id.eq(id))
            .order(contents::order.asc())
            .then_order_by(contents::id.asc())
            .select((
                contents::id,
                contents::order,
                contents::content_type,
                contents::data,
            ))
            .load::<(i32, i32, String, Option<String>)>(conn)
            .await?;
        let mut workspace_contents = Vec::with_capacity(content_rows.len());

        for (id, order, content_type, data) in content_rows {
            let processing = load_latest_content_processing(conn, data.as_deref()).await?;
            let display_state = content_display_state(&content_type, data.as_deref(), &processing);
            workspace_contents.push(TeacherCourseWorkspaceContent {
                id,
                order,
                content_type,
                data_present: data
                    .as_deref()
                    .map(str::trim)
                    .is_some_and(|value| !value.is_empty()),
                publication_status: teacher_content_publication_status(course_lifecycle_status),
                display_state,
                processing_status: processing.as_ref().map(|(status, _)| status.clone()),
                processing_error: processing.and_then(|(_, error)| error),
            });
        }

        result.push(TeacherCourseWorkspaceChapter {
            id,
            title,
            order,
            contents: workspace_contents,
        });
    }

    Ok(result)
}

fn teacher_content_publication_status(course_lifecycle_status: &str) -> String {
    format!("inherits_course_{}", course_lifecycle_status)
}
