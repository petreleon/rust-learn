// ── reward_policy_repository ──

#[actix_web::test]
async fn test_policy_create_and_list() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("policy_course")).await;

    let policy = reward_policy_repository::create_policy(
        &mut conn,
        NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(50),
            multiplier: BigDecimal::from(1),
            max_payout: None,
            cooldown_seconds: 3600,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(policy.event_type, REWARD_EVENT_COURSE_COMPLETION);
    assert_eq!(policy.course_id, Some(course.id));
    assert!(policy.active);

    let policies = reward_policy_repository::list_policies(
        &mut conn,
        reward_policy_repository::RewardPolicyFilter {
            course_id: Some(course.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(!policies.is_empty());
    assert_eq!(policies[0].course_id, Some(course.id));
}

#[actix_web::test]
async fn test_policy_deactivate() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("policy_deact_course")).await;

    let policy = reward_policy_repository::create_policy(
        &mut conn,
        NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(25),
            multiplier: BigDecimal::from(1),
            max_payout: None,
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        },
    )
    .await
    .unwrap();
    assert!(policy.active);

    let count = reward_policy_repository::deactivate_active_policies(
        &mut conn,
        REWARD_POLICY_SCOPE_COURSE,
        None,
        Some(course.id),
        REWARD_EVENT_COURSE_COMPLETION,
        chrono::Utc::now(),
    )
    .await
    .unwrap();
    assert!(count >= 1);

    let policies = reward_policy_repository::list_policies(
        &mut conn,
        reward_policy_repository::RewardPolicyFilter {
            course_id: Some(course.id),
            active: Some(true),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(policies.len(), 0);
}
