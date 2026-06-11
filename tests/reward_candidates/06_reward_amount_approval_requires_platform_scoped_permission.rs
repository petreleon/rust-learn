#[actix_web::test]
async fn reward_amount_approval_requires_platform_scoped_permission() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PlatformAmountScopeCourse")).await;
    let teacher = create_user_helper(&mut conn, "platform_amount_teacher").await;
    let student = create_user_helper(&mut conn, "platform_amount_student").await;
    let platform_reviewer = create_user_helper(&mut conn, "platform_amount_reviewer").await;
    let course_scoped_reviewer = create_user_helper(&mut conn, "course_amount_reviewer").await;

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let platform_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("CENTRAL_AMOUNT_REVIEWER"),
        &[Permissions::APPROVE_REWARD_AMOUNT],
    )
    .await;
    let course_role_id = create_custom_course_role(
        &mut conn,
        &unique_string("COURSE_AMOUNT_REVIEWER"),
        &[Permissions::APPROVE_REWARD_AMOUNT],
    )
    .await;
    assign_platform_role_id(&mut conn, platform_reviewer.id(), platform_role_id).await;
    assign_course_role_id(
        &mut conn,
        course_scoped_reviewer.id(),
        course.id,
        course_role_id,
    )
    .await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("platform_amount_scope")),
    )
    .await
    .expect("teacher should submit reward candidate");
    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("course approval complete".to_string()),
        },
    )
    .await
    .expect("teacher should approve candidate before amount review");

    let course_scoped_denied = decide_reward_amount(
        &mut conn,
        course_scoped_reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(20)),
            decision_reason: Some("course-scoped amount attempt".to_string()),
        },
    )
    .await
    .expect_err("course-scoped amount permission must not approve platform amount");
    assert!(matches!(
        course_scoped_denied,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_REWARD_AMOUNT.to_string()
    ));

    let approved = decide_reward_amount(
        &mut conn,
        platform_reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(20)),
            decision_reason: Some("platform amount approved".to_string()),
        },
    )
    .await
    .expect("platform-scoped amount permission should approve reward amount");
    assert_eq!(approved.status, REWARD_STATUS_AMOUNT_APPROVED);
    assert_eq!(
        approved.amount_reviewer_user_id,
        Some(platform_reviewer.id())
    );
}
