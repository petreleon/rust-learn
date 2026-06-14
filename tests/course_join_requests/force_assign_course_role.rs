async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("course role not found");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

#[actix_web::test]
async fn organization_student_can_request_and_course_teacher_can_approve_join() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("JoinOrg")).await;
    let course = create_course(&mut conn, &unique_string("JoinCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;

    let student = create_user_helper(&mut conn, "join_student").await;
    let teacher = create_user_helper(&mut conn, "join_teacher").await;
    force_assign_organization_role(&mut conn, student.id(), organization.id, "STUDENT").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;

    let join_request = request_course_join(&mut conn, student.id(), course.id)
        .await
        .expect("organization student should request linked course join");
    assert_eq!(join_request.status, COURSE_JOIN_STATUS_PENDING);

    let duplicate = request_course_join(&mut conn, student.id(), course.id)
        .await
        .expect("duplicate pending join request should be idempotent");
    assert_eq!(duplicate.id, join_request.id);

    let approved = decide_course_join_request(
        &mut conn,
        teacher.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: Some("approved by teacher".to_string()),
        },
    )
    .await
    .expect("course teacher should approve join request");
    assert_eq!(approved.status, COURSE_JOIN_STATUS_APPROVED);
    assert_eq!(approved.reviewer_user_id, Some(teacher.id()));

    let enrolled = user_permission_course_request(
        &mut conn,
        student.id(),
        course.id,
        &Permissions::VIEW_COURSE.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(enrolled, "approved student should receive course bundle");
}

#[actix_web::test]
async fn platform_user_can_request_and_organization_admin_can_approve_join() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("JoinApprovalOrg")).await;
    let course = create_course(&mut conn, &unique_string("JoinApprovalCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;

    let requester = create_user_helper(&mut conn, "join_platform_user").await;
    let org_admin = create_user_helper(&mut conn, "join_org_admin").await;
    force_assign_platform_role(&mut conn, requester.id(), "USER").await;
    force_assign_organization_role(&mut conn, org_admin.id(), organization.id, "ADMIN").await;

    let join_request = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("platform user should request course join");

    let approved = decide_course_join_request(
        &mut conn,
        org_admin.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect("organization admin should approve linked course join request");
    assert_eq!(approved.status, COURSE_JOIN_STATUS_APPROVED);
}

#[actix_web::test]
async fn user_without_scoped_join_permission_cannot_request() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "join_denied").await;
    let course = create_course(&mut conn, &unique_string("JoinDeniedCourse")).await;

    let denied = request_course_join(&mut conn, user.id(), course.id)
        .await
        .expect_err("user without join permission should be denied");
    assert!(matches!(denied, CourseEnrollmentError::PermissionDenied(_)));
}
