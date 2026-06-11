#[actix_web::test]
async fn test_fraud_block_revoke() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_revoke_creator").await;
    let revoker = create_user_helper(&mut conn, "fraud_revoker").await;
    let teacher = create_user_helper(&mut conn, "fraud_revoke_teacher").await;

    let block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "to revoke".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    assert!(block.revoked_at.is_none());

    let now = Utc::now();
    let revoked = reward_fraud_block_repository::revoke_reward_fraud_block(
        &mut conn,
        block.id,
        revoker.id(),
        now,
    )
    .await
    .unwrap();

    assert!(revoked.revoked_at.is_some());
    assert_eq!(revoked.revoked_by_user_id, Some(revoker.id()));
}

#[actix_web::test]
async fn test_fraud_block_active_filter() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_active_creator").await;
    let teacher = create_user_helper(&mut conn, "fraud_active_teacher").await;

    let block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "active filter test".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    let (active_blocks, _) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter {
            active: Some(true),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(active_blocks.iter().any(|b| b.id == block.id));

    reward_fraud_block_repository::revoke_reward_fraud_block(
        &mut conn,
        block.id,
        creator.id(),
        Utc::now(),
    )
    .await
    .unwrap();

    let (active_after, _) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter {
            active: Some(true),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(!active_after.iter().any(|b| b.id == block.id));
}

// ── reward_execution_job_repository ──

#[actix_web::test]
async fn test_execution_job_enqueue_and_find() {
    let mut conn = setup_conn().await;
    let student = create_user_helper(&mut conn, "exec_student").await;
    let course = create_course(&mut conn, &unique_string("exec_course")).await;

    let candidate = reward_candidate_repository::create_candidate(
        &mut conn,
        new_candidate(course.id, student.id(), &unique_string("exec_cand")),
    )
    .await
    .unwrap();

    let job =
        reward_execution_job_repository::enqueue_reward_execution_job(&mut conn, candidate.id)
            .await
            .unwrap();

    assert_eq!(job.reward_candidate_id, candidate.id);
    assert_eq!(job.status, REWARD_EXECUTION_STATUS_QUEUED);

    let found = reward_execution_job_repository::find_job_by_candidate(&mut conn, candidate.id)
        .await
        .unwrap()
        .expect("execution job not found");
    assert_eq!(found.id, job.id);
}
