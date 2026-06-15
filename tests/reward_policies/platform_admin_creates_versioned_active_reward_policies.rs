use crate::support::*;

#[actix_web::test]
async fn platform_admin_creates_versioned_active_reward_policies() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let admin = create_user_helper(&mut conn, "reward_policy_admin").await;
    assign_platform_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    drop(conn);

    let policies = reward_policy_use_case(&pool);
    let first = policies
        .create_reward_policy(admin.id(), platform_policy_request("10"))
        .await
        .expect("admin should create first reward policy");
    assert!(first.version >= 1);
    assert!(first.active);

    let second = policies
        .create_reward_policy(admin.id(), platform_policy_request("15"))
        .await
        .expect("admin should create second reward policy version");
    assert_eq!(second.version, first.version + 1);
    assert!(second.active);

    let active = policies
        .list_reward_policies(
            admin.id(),
            ListRewardPoliciesQuery {
                scope_type: Some(RewardPolicyScope::Platform),
                event_type: Some(RewardPolicyEventType::CourseCompletion),
                active: Some(true),
                ..Default::default()
            },
        )
        .await
        .expect("admin should list active policies");
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, second.id);

    let inactive = policies
        .list_reward_policies(
            admin.id(),
            ListRewardPoliciesQuery {
                scope_type: Some(RewardPolicyScope::Platform),
                event_type: Some(RewardPolicyEventType::CourseCompletion),
                active: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("admin should list inactive policies");
    assert!(inactive.iter().any(|policy| policy.id == first.id));
}

#[actix_web::test]
async fn platform_moderator_cannot_set_reward_policy() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let moderator = create_user_helper(&mut conn, "reward_policy_moderator").await;
    assign_platform_role_to_user(&mut conn, moderator.id(), Roles::MODERATOR)
        .await
        .expect("failed to assign MODERATOR role");
    drop(conn);

    let denied = reward_policy_use_case(&pool)
        .create_reward_policy(moderator.id(), platform_policy_request("10"))
        .await
        .expect_err("moderator should not set reward policy");
    assert!(matches!(denied, RewardPolicyError::PermissionDenied(_)));
}

#[actix_web::test]
async fn course_policy_requires_course_scope_and_can_explicitly_allow_mint() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let admin = create_user_helper(&mut conn, "reward_policy_course_admin").await;
    let course = create_course(&mut conn, &unique_string("RewardPolicyCourse")).await;
    let organization = create_organization(&mut conn, &unique_string("RewardPolicyOrg")).await;
    assign_platform_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    drop(conn);

    let policies = reward_policy_use_case(&pool);
    let policy = policies
        .create_reward_policy(
            admin.id(),
            CreateRewardPolicyCommand {
                scope_type: RewardPolicyScope::Course,
                organization_id: Some(organization.id),
                course_id: Some(course.id),
                event_type: RewardPolicyEventType::CourseCompletion,
                token_amount: BigDecimal::from(20),
                multiplier: Some(BigDecimal::from_str("1.25").expect("valid multiplier")),
                max_payout: Some(BigDecimal::from(50)),
                cooldown_seconds: Some(3_600),
                payment_strategy: RewardPaymentStrategy::Mint,
                active: Some(true),
            },
        )
        .await
        .expect("admin should create course reward policy");

    assert_eq!(policy.scope_type, RewardPolicyScope::Course);
    assert_eq!(policy.course_id, Some(course.id));
    assert_eq!(policy.organization_id, Some(organization.id));
    assert_eq!(policy.payment_strategy, RewardPaymentStrategy::Mint);

    let invalid = policies
        .create_reward_policy(
            admin.id(),
            CreateRewardPolicyCommand {
                scope_type: RewardPolicyScope::Course,
                organization_id: None,
                course_id: None,
                event_type: RewardPolicyEventType::CourseCompletion,
                token_amount: BigDecimal::from(20),
                multiplier: None,
                max_payout: None,
                cooldown_seconds: None,
                payment_strategy: RewardPaymentStrategy::TreasuryTransfer,
                active: Some(true),
            },
        )
        .await
        .expect_err("course policy should require course_id");
    assert!(matches!(invalid, RewardPolicyError::InvalidInput(_)));
}
