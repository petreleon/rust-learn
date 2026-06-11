#[actix_web::test]
async fn platform_amount_permission_cannot_submit_or_approve_candidate() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("RewardBoundaryOrg")).await;
    let course = create_course(&mut conn, &unique_string("RewardBoundaryCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;
    let teacher = create_user_helper(&mut conn, "reward_boundary_teacher").await;
    let student = create_user_helper(&mut conn, "reward_boundary_student").await;
    let reviewer = create_user_helper(&mut conn, "reward_boundary_reviewer").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let amount_only_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("AMOUNT_ONLY_REVIEWER"),
        &[Permissions::APPROVE_REWARD_AMOUNT],
    )
    .await;
    assign_platform_role_id(&mut conn, reviewer.id(), amount_only_role_id).await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let denied_submit = submit_course_reward_candidate(
        &mut conn,
        reviewer.id(),
        course.id,
        reward_request(student.id(), &unique_string("denied_reward")),
    )
    .await
    .expect_err("platform amount reviewer must not submit course reward candidates");
    assert!(matches!(
        denied_submit,
        RewardCandidateError::PermissionDenied(_)
    ));

    let denied_org_submit = submit_organization_reward_candidate(
        &mut conn,
        reviewer.id(),
        organization.id,
        course.id,
        reward_request(student.id(), &unique_string("denied_org_reward")),
    )
    .await
    .expect_err("platform amount reviewer must not submit organization reward candidates");
    assert!(matches!(
        denied_org_submit,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string()
    ));

    let denied_student_submit = submit_course_reward_candidate(
        &mut conn,
        student.id(),
        course.id,
        reward_request(student.id(), &unique_string("student_self_reward")),
    )
    .await
    .expect_err("student activity evidence must not authorize self-submission");
    assert!(matches!(
        denied_student_submit,
        RewardCandidateError::PermissionDenied(_)
    ));

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("teacher_reward")),
    )
    .await
    .expect("teacher should submit reward candidate");

    let denied_teacher_decision = decide_reward_candidate_by_teacher(
        &mut conn,
        reviewer.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect_err("platform amount reviewer must not approve reward candidates");
    assert!(matches!(
        denied_teacher_decision,
        RewardCandidateError::PermissionDenied(_)
    ));

    let denied_teacher_rejection = decide_reward_candidate_by_teacher(
        &mut conn,
        reviewer.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "rejected".to_string(),
            decision_reason: Some("amount reviewer attempted candidate rejection".to_string()),
        },
    )
    .await
    .expect_err("platform amount reviewer must not reject reward candidates");
    assert!(matches!(
        denied_teacher_rejection,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()
    ));

    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect("teacher approval should succeed");

    let teacher_amount_decision = decide_reward_amount(
        &mut conn,
        teacher.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(12)),
            decision_reason: None,
        },
    )
    .await
    .expect_err("course teacher must not set platform reward amount");
    assert!(matches!(
        teacher_amount_decision,
        RewardCandidateError::PermissionDenied(_)
    ));
}
