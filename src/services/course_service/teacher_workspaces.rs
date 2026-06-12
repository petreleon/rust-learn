pub async fn get_teacher_course_workspace(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<TeacherCourseWorkspaceResponse, TeacherCourseDashboardError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
    if !permissions.has_teacher_access() {
        return Err(TeacherCourseDashboardError::PermissionDenied(
            "teaching course access".to_string(),
        ));
    }

    let teacher_roles = load_actor_course_roles(conn, actor_user_id, course.id).await?;
    let publication = TeacherCoursePublicationSummary {
        course_lifecycle_status: course.lifecycle_status.clone(),
        content_publication_status_supported: false,
    };
    let chapters =
        load_teacher_course_workspace_chapters(conn, course.id, &course.lifecycle_status).await?;
    let course = build_teacher_course_dashboard_item(conn, course, permissions).await?;

    Ok(TeacherCourseWorkspaceResponse {
        course,
        teacher_roles,
        publication,
        chapters,
    })
}

pub async fn get_teacher_course_enrollment_workspace(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    enrollment_query: TeacherCourseEnrollmentQuery,
) -> Result<TeacherCourseEnrollmentWorkspaceResponse, TeacherCourseDashboardError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
    if !permissions.can_manage_enrollments {
        return Err(TeacherCourseDashboardError::PermissionDenied(
            "course enrollment management".to_string(),
        ));
    }

    let can_manage_enrollments = permissions.can_manage_enrollments;
    let teacher_roles = load_actor_course_roles(conn, actor_user_id, course.id).await?;
    let join_requests = load_teacher_course_join_request_page(
        conn,
        course.id,
        &enrollment_query,
        can_manage_enrollments,
    )
    .await?;
    let roster = load_teacher_course_roster_page(conn, course.id, can_manage_enrollments).await?;
    let reward_eligibility =
        load_teacher_course_reward_eligibility_summary(conn, course.id).await?;
    let course = build_teacher_course_dashboard_item(conn, course, permissions).await?;

    Ok(TeacherCourseEnrollmentWorkspaceResponse {
        course,
        teacher_roles,
        join_requests,
        roster,
        progress_supported: true,
        reward_eligibility_supported: reward_eligibility.supported,
        reward_eligibility,
    })
}

pub async fn get_teacher_course_students(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<TeacherCourseStudentsResponse, TeacherCourseDashboardError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)?;
    let permissions = build_teacher_course_permissions(conn, actor_user_id, course.id).await?;
    let can_view_students = permissions.can_manage_enrollments
        || permissions.can_view_reward_candidates
        || permissions.can_approve_reward_candidates;
    if !can_view_students {
        return Err(TeacherCourseDashboardError::PermissionDenied(
            "course student progress".to_string(),
        ));
    }

    let teacher_roles = load_actor_course_roles(conn, actor_user_id, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let course_reward_eligibility =
        load_teacher_course_reward_eligibility_summary(conn, course.id).await?;
    let roster =
        load_teacher_course_roster_page(conn, course.id, permissions.can_manage_enrollments)
            .await?;
    let mut students = Vec::with_capacity(roster.learners.len());
    for learner in roster.learners {
        let rewards =
            load_teacher_student_reward_progress(conn, course.id, learner.user.id).await?;
        let progress = load_teacher_student_lesson_progress(
            conn,
            course.id,
            learner.user.id,
            content.content_count,
        )
        .await?;
        let reward_eligibility = teacher_student_reward_eligibility_from_count(
            &course_reward_eligibility,
            rewards.reward_candidate_count,
        );
        students.push(TeacherCourseStudentProgressItem {
            user: learner.user,
            roles: learner.roles,
            access_state: learner.access_state,
            latest_join_request_status: learner.latest_join_request_status,
            progress,
            reward_eligibility,
            rewards,
        });
    }

    let total = students.len() as i64;
    let course = build_teacher_course_dashboard_item(conn, course, permissions).await?;

    Ok(TeacherCourseStudentsResponse {
        course,
        teacher_roles,
        students,
        total,
        progress_supported: true,
        reward_eligibility_supported: course_reward_eligibility.supported,
        reward_eligibility: course_reward_eligibility,
        reward_evidence_supported: true,
    })
}
