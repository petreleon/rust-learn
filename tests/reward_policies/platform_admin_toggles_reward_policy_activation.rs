use crate::support::*;

#[actix_web::test]
async fn platform_admin_toggles_policy_activation_and_reads_audit() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let admin = create_user_helper(&mut conn, "reward_policy_toggle_admin").await;
    let course = create_course(&mut conn, &unique_string("RewardPolicyToggleCourse")).await;
    assign_platform_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    drop(conn);

    let policies = reward_policy_use_case(&pool);
    let first = policies
        .create_reward_policy(admin.id(), course_policy_request(course.id, "10"))
        .await
        .expect("admin should create first reward policy");
    let second = policies
        .create_reward_policy(admin.id(), course_policy_request(course.id, "15"))
        .await
        .expect("admin should create second reward policy");
    assert!(second.active);

    let reactivated = policies
        .update_reward_policy_activation(
            admin.id(),
            UpdateRewardPolicyActivationCommand {
                active: true,
                policy_id: first.id,
            },
        )
        .await
        .expect("admin should reactivate first policy");
    assert!(reactivated.active);

    let active = policies
        .list_reward_policies(
            admin.id(),
            ListRewardPoliciesQuery {
                active: Some(true),
                course_id: Some(course.id),
                event_type: Some(RewardPolicyEventType::CourseCompletion),
                scope_type: Some(RewardPolicyScope::Course),
                ..Default::default()
            },
        )
        .await
        .expect("admin should list active policies");
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, first.id);

    let first_audit = policies
        .list_reward_policy_audit(admin.id(), first.id)
        .await
        .expect("admin should read first policy audit");
    assert!(first_audit
        .iter()
        .any(|event| event.event_type == RewardPolicyAuditEventType::Created));
    assert!(first_audit.iter().any(|event| {
        event.event_type == RewardPolicyAuditEventType::Deactivated
            && event.previous_active == Some(true)
            && !event.new_active
    }));
    assert!(first_audit.iter().any(|event| {
        event.event_type == RewardPolicyAuditEventType::Activated
            && event.previous_active == Some(false)
            && event.new_active
    }));

    let second_audit = policies
        .list_reward_policy_audit(admin.id(), second.id)
        .await
        .expect("admin should read second policy audit");
    assert!(second_audit
        .iter()
        .any(|event| event.event_type == RewardPolicyAuditEventType::Deactivated));
}

#[actix_web::test]
async fn moderator_cannot_toggle_policy_activation_or_read_policy_audit() {
    let pool = setup_pool();
    let mut conn = setup_conn(&pool).await;
    let admin = create_user_helper(&mut conn, "reward_policy_toggle_owner").await;
    let moderator = create_user_helper(&mut conn, "reward_policy_toggle_moderator").await;
    let course = create_course(&mut conn, &unique_string("RewardPolicyToggleDeniedCourse")).await;
    assign_platform_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    assign_platform_role_to_user(&mut conn, moderator.id(), Roles::MODERATOR)
        .await
        .expect("failed to assign MODERATOR role");
    drop(conn);

    let policies = reward_policy_use_case(&pool);
    let policy = policies
        .create_reward_policy(admin.id(), course_policy_request(course.id, "10"))
        .await
        .expect("admin should create reward policy");

    let denied = policies
        .update_reward_policy_activation(
            moderator.id(),
            UpdateRewardPolicyActivationCommand {
                active: false,
                policy_id: policy.id,
            },
        )
        .await
        .expect_err("moderator should not toggle reward policy");
    assert!(matches!(denied, RewardPolicyError::PermissionDenied(_)));

    let audit_denied = policies
        .list_reward_policy_audit(moderator.id(), policy.id)
        .await
        .expect_err("moderator should not read policy audit");
    assert!(matches!(
        audit_denied,
        RewardPolicyError::PermissionDenied(_)
    ));
}

fn course_policy_request(course_id: i32, amount: &str) -> CreateRewardPolicyCommand {
    CreateRewardPolicyCommand {
        scope_type: RewardPolicyScope::Course,
        organization_id: None,
        course_id: Some(course_id),
        event_type: RewardPolicyEventType::CourseCompletion,
        token_amount: BigDecimal::from_str(amount).expect("valid amount"),
        multiplier: Some(BigDecimal::from(1)),
        max_payout: Some(BigDecimal::from(100)),
        cooldown_seconds: Some(86_400),
        payment_strategy: RewardPaymentStrategy::TreasuryTransfer,
        active: Some(true),
    }
}
