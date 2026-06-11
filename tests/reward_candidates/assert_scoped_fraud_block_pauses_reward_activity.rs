async fn assert_scoped_fraud_block_pauses_reward_activity(scope: FraudBlockScopeUnderTest) {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("ScopedBlockOrg")).await;
    let course = create_course(&mut conn, &unique_string("ScopedBlockCourse")).await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;
    let teacher = create_user_helper(&mut conn, "scoped_block_teacher").await;
    let student = create_user_helper(&mut conn, "scoped_block_student").await;
    let second_student = create_user_helper(&mut conn, "scoped_block_student_two").await;
    let reviewer = create_user_helper(&mut conn, "scoped_block_reviewer").await;
    let admin = create_user_helper(&mut conn, "scoped_block_admin").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    force_assign_course_role(&mut conn, second_student.id(), course.id, "STUDENT").await;
    force_assign_platform_role(&mut conn, reviewer.id(), "MODERATOR").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    let reward_policy_id =
        create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION)
            .await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("scoped_block_candidate")),
    )
    .await
    .expect("teacher should submit before scoped fraud block is active");

    let block_request =
        scoped_fraud_block_request(scope, organization.id, course.id, reward_policy_id);
    let active_block = create_reward_fraud_block(&mut conn, admin.id(), block_request.clone())
        .await
        .expect("admin should create scoped fraud block");

    let denied_submission = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(
            second_student.id(),
            &unique_string("scoped_block_submission"),
        ),
    )
    .await
    .expect_err("active scoped fraud block should prevent new reward submission");
    assert!(matches!(
        denied_submission,
        RewardCandidateError::InvalidStatus(message)
            if message.contains(expected_block_message(scope))
    ));

    let denied_teacher_approval = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("student completed the course".to_string()),
        },
    )
    .await
    .expect_err("active scoped fraud block should prevent teacher approval");
    assert!(matches!(
        denied_teacher_approval,
        RewardCandidateError::InvalidStatus(message)
            if message.contains(expected_block_message(scope))
    ));

    revoke_reward_fraud_block(&mut conn, admin.id(), active_block.id)
        .await
        .expect("admin should revoke scoped fraud block");

    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("student completed the course".to_string()),
        },
    )
    .await
    .expect("teacher approval should resume after scoped block revocation");

    create_reward_fraud_block(&mut conn, admin.id(), block_request)
        .await
        .expect("admin should recreate scoped fraud block");

    let denied_amount = decide_reward_amount(
        &mut conn,
        reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: Some("amount approved by central reviewer".to_string()),
        },
    )
    .await
    .expect_err("active scoped fraud block should prevent amount approval");
    assert!(matches!(
        denied_amount,
        RewardCandidateError::InvalidStatus(message)
            if message.contains(expected_block_message(scope))
    ));
}

#[actix_web::test]
async fn organization_course_and_policy_fraud_blocks_pause_reward_activity() {
    for scope in [
        FraudBlockScopeUnderTest::Organization,
        FraudBlockScopeUnderTest::Course,
        FraudBlockScopeUnderTest::RewardPolicy,
    ] {
        assert_scoped_fraud_block_pauses_reward_activity(scope).await;
    }
}
