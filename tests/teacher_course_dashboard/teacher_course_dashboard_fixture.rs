struct TeacherDashboardFixture {
    pool: DbPool,
    teacher: User,
    student: User,
    pending_learner: User,
    outsider: User,
    org: Organization,
    course: Course,
}

async fn setup_teacher_dashboard_fixture() -> TeacherDashboardFixture {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let teacher = create_test_user(&mut conn, "teacher_dashboard_teacher").await;
    let student = create_test_user(&mut conn, "teacher_dashboard_student").await;
    let pending_learner = create_test_user(&mut conn, "teacher_dashboard_pending").await;
    let outsider = create_test_user(&mut conn, "teacher_dashboard_outsider").await;
    assign_platform_role(&mut conn, outsider.id(), "USER").await;

    let org = create_organization(&mut conn, &unique_string("TeacherDashboardOrg")).await;
    let course = create_course(&mut conn, &unique_string("TeacherDashboardCourse")).await;
    let hidden_course = create_course(&mut conn, &unique_string("HiddenTeacherCourse")).await;
    publish_course(&mut conn, course.id).await;
    publish_course(&mut conn, hidden_course.id).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;

    let chapter_id = create_chapter(&mut conn, course.id, "Dashboard chapter").await;
    let content_id = create_content(
        &mut conn,
        chapter_id,
        "article",
        Some("Workspace lesson text"),
    )
    .await;
    create_course_progress(&mut conn, student.id(), course.id, content_id).await;
    create_reward_policy(&mut conn, course.id).await;
    create_join_request(
        &mut conn,
        pending_learner.id(),
        course.id,
        COURSE_JOIN_STATUS_PENDING,
    )
    .await;
    create_join_request(
        &mut conn,
        outsider.id(),
        course.id,
        COURSE_JOIN_STATUS_WAITLISTED,
    )
    .await;
    create_join_request(
        &mut conn,
        student.id(),
        course.id,
        COURSE_JOIN_STATUS_APPROVED,
    )
    .await;

    for status in [
        REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        REWARD_STATUS_TEACHER_APPROVED,
        REWARD_STATUS_FAILED,
    ] {
        create_reward_candidate(&mut conn, course.id, student.id(), teacher.id(), status).await;
    }
    drop(conn);

    TeacherDashboardFixture {
        pool,
        teacher,
        student,
        pending_learner,
        outsider,
        org,
        course,
    }
}

async fn create_course_progress(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    content_id: i32,
) {
    diesel::insert_into(course_progress::table)
        .values((
            course_progress::user_id.eq(user_id),
            course_progress::course_id.eq(course_id),
            course_progress::content_id.eq(content_id),
        ))
        .execute(conn)
        .await
        .expect("failed to save course progress");
}
