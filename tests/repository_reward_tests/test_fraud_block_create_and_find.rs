// ── reward_fraud_block_repository ──

#[actix_web::test]
async fn test_fraud_block_create_and_find() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_creator").await;
    let teacher = create_user_helper(&mut conn, "fraud_teacher").await;

    let block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "suspicious activity".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(block.scope_type, REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    assert_eq!(block.teacher_user_id, Some(teacher.id()));

    let found = reward_fraud_block_repository::find_reward_fraud_block(&mut conn, block.id)
        .await
        .unwrap();
    assert_eq!(found.id, block.id);
}

#[actix_web::test]
async fn test_fraud_block_list_and_filter() {
    let mut conn = setup_conn().await;
    let creator = create_user_helper(&mut conn, "fraud_list_creator").await;

    let fraud_teacher = create_user_helper(&mut conn, "fraud_t1").await;

    let teacher_block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
            teacher_user_id: Some(fraud_teacher.id()),
            organization_id: None,
            course_id: None,
            reward_policy_id: None,
            reason: "teacher fraud".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    let fraud_course = create_course(&mut conn, &unique_string("fraud_course")).await;

    let course_block = reward_fraud_block_repository::create_reward_fraud_block(
        &mut conn,
        NewRewardFraudBlock {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: Some(fraud_course.id),
            reward_policy_id: None,
            reason: "course fraud".to_string(),
            evidence_reference: None,
            created_by_user_id: creator.id(),
            expires_at: None,
        },
    )
    .await
    .unwrap();

    let (_blocks, total) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter::default(),
    )
    .await
    .unwrap();
    assert!(total >= 2);

    let (teacher_blocks, teacher_total) = reward_fraud_block_repository::list_reward_fraud_blocks(
        &mut conn,
        RewardFraudBlockFilter {
            scope_type: Some(REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(teacher_total >= 1);
    assert!(teacher_blocks.iter().any(|b| b.id == teacher_block.id));
    assert!(!teacher_blocks.iter().any(|b| b.id == course_block.id));
}
