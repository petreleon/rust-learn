#[actix_web::test]
async fn organization_admin_can_remove_course_enrollment() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("RemoveEnrollmentOrg")).await;
    let course = create_course(&mut conn, &unique_string("RemoveEnrollmentCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;

    let requester = create_user_helper(&mut conn, "remove_enrollment_student").await;
    let org_admin = create_user_helper(&mut conn, "remove_enrollment_admin").await;
    force_assign_platform_role(&mut conn, requester.id(), "USER").await;
    force_assign_organization_role(&mut conn, org_admin.id(), organization.id, "ADMIN").await;

    let join_request = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("requester should create join request");
    decide_course_join_request(
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

    let removal = remove_course_enrollment(&mut conn, org_admin.id(), course.id, requester.id())
        .await
        .expect("organization admin should remove course enrollment");
    assert!(removal.removed);

    let still_enrolled = has_course_permission(
        &mut conn,
        requester.id(),
        course.id,
        &Permissions::VIEW_COURSE.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        !still_enrolled,
        "removed student should lose course-scoped student bundle"
    );
}

#[actix_web::test]
async fn course_student_cannot_remove_course_enrollment() {
    let mut conn = setup_conn().await;
    let requester = create_user_helper(&mut conn, "remove_denied_requester").await;
    let teacher = create_user_helper(&mut conn, "remove_denied_teacher").await;
    let reviewer = create_user_helper(&mut conn, "remove_denied_student_reviewer").await;
    let course = create_course(&mut conn, &unique_string("RemoveDeniedCourse")).await;
    force_assign_platform_role(&mut conn, requester.id(), "USER").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, reviewer.id(), course.id, "STUDENT").await;

    let join_request = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("requester should create join request");
    decide_course_join_request(
        &mut conn,
        teacher.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect("teacher should approve join request");

    let denied = remove_course_enrollment(&mut conn, reviewer.id(), course.id, requester.id())
        .await
        .expect_err("course student should not remove enrollments");
    assert!(matches!(denied, CourseEnrollmentError::PermissionDenied(_)));
}
