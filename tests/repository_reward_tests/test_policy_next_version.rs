use crate::support::*;

#[actix_web::test]
async fn test_policy_next_version() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("policy_ver_course")).await;

    let v1 = reward_policy_repository::next_policy_version(
        &mut conn,
        REWARD_POLICY_SCOPE_COURSE,
        None,
        Some(course.id),
        REWARD_EVENT_COURSE_COMPLETION,
    )
    .await
    .unwrap();
    assert_eq!(v1, 1);

    reward_policy_repository::create_policy(
        &mut conn,
        NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: v1,
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

    let v2 = reward_policy_repository::next_policy_version(
        &mut conn,
        REWARD_POLICY_SCOPE_COURSE,
        None,
        Some(course.id),
        REWARD_EVENT_COURSE_COMPLETION,
    )
    .await
    .unwrap();
    assert_eq!(v2, 2);
}
