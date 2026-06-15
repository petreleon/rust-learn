use crate::force_assign_course_role::*;
use crate::support::*;

#[actix_web::test]
async fn course_student_cannot_approve_join_request() {
    let mut conn = setup_conn().await;
    let requester = create_user_helper(&mut conn, "join_approval_requester").await;
    let reviewer = create_user_helper(&mut conn, "join_student_reviewer").await;
    let course = create_course(&mut conn, &unique_string("JoinReviewerDeniedCourse")).await;
    force_assign_platform_role(&mut conn, requester.id(), "USER").await;
    force_assign_course_role(&mut conn, reviewer.id(), course.id, "STUDENT").await;

    let join_request = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("requester should create join request");

    let denied = decide_course_join_request(
        &mut conn,
        reviewer.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect_err("course student should not approve join request");
    assert!(matches!(denied, CourseEnrollmentError::PermissionDenied(_)));
}

#[actix_web::test]
async fn course_teacher_can_waitlist_then_approve_join_request() {
    let mut conn = setup_conn().await;
    let requester = create_user_helper(&mut conn, "join_waitlist_requester").await;
    let teacher = create_user_helper(&mut conn, "join_waitlist_teacher").await;
    let course = create_course(&mut conn, &unique_string("JoinWaitlistCourse")).await;
    force_assign_platform_role(&mut conn, requester.id(), "USER").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;

    let join_request = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("requester should create join request");

    let waitlisted = decide_course_join_request(
        &mut conn,
        teacher.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_WAITLISTED.to_string(),
            decision_reason: Some("capacity is full".to_string()),
        },
    )
    .await
    .expect("course teacher should waitlist join request");
    assert_eq!(waitlisted.status, COURSE_JOIN_STATUS_WAITLISTED);

    let duplicate = request_course_join(&mut conn, requester.id(), course.id)
        .await
        .expect("waitlisted join request should remain open");
    assert_eq!(duplicate.id, join_request.id);
    assert_eq!(duplicate.status, COURSE_JOIN_STATUS_WAITLISTED);

    let approved = decide_course_join_request(
        &mut conn,
        teacher.id(),
        course.id,
        join_request.id,
        CourseJoinDecisionRequest {
            status: COURSE_JOIN_STATUS_APPROVED.to_string(),
            decision_reason: Some("seat opened".to_string()),
        },
    )
    .await
    .expect("course teacher should approve waitlisted join request");
    assert_eq!(approved.status, COURSE_JOIN_STATUS_APPROVED);

    let enrolled = has_course_permission(
        &mut conn,
        requester.id(),
        course.id,
        &Permissions::VIEW_COURSE.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(enrolled, "approved waitlisted user should be enrolled");
}
