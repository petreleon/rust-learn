#[actix_web::test]
async fn platform_permissions_create_and_revoke_reward_fraud_blocks() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "fraud_block_admin").await;
    let moderator = create_user_helper(&mut conn, "fraud_block_moderator").await;
    let teacher = create_user_helper(&mut conn, "fraud_block_teacher").await;
    let org_operator = create_user_helper(&mut conn, "fraud_block_org_operator").await;
    let delegated_platform_auditor =
        create_user_helper(&mut conn, "fraud_block_delegated_platform_auditor").await;
    let delegated_org_operator =
        create_user_helper(&mut conn, "fraud_block_delegated_org_operator").await;
    let organization = create_organization(&mut conn, &unique_string("FraudBlockOrg")).await;
    let course = create_course(&mut conn, &unique_string("FraudBlockCourse")).await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_platform_role(&mut conn, moderator.id(), "MODERATOR").await;
    force_assign_organization_role(&mut conn, org_operator.id(), organization.id, "ADMIN").await;
    grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: delegated_platform_auditor.id(),
            permission: Permissions::VIEW_REWARD_AUDIT.to_string(),
            scope_type: DELEGATED_SCOPE_PLATFORM.to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("temporary fraud-audit coverage".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate platform fraud audit notifications");
    grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: delegated_org_operator.id(),
            permission: Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
            scope_type: DELEGATED_SCOPE_ORGANIZATION.to_string(),
            organization_id: Some(organization.id),
            course_id: None,
            reason: Some("temporary organization reward report coverage".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate organization reward report notifications");

    let denied = create_reward_fraud_block(
        &mut conn,
        moderator.id(),
        teacher_block_request(teacher.id()),
    )
    .await
    .expect_err("moderator should not get teacher fraud block permission by default");
    assert!(matches!(denied, RewardFraudBlockError::PermissionDenied(_)));

    let teacher_block =
        create_reward_fraud_block(&mut conn, admin.id(), teacher_block_request(teacher.id()))
            .await
            .expect("admin should block teacher reward activity");
    assert_eq!(teacher_block.scope_type, REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    assert_eq!(teacher_block.teacher_user_id, Some(teacher.id()));
    assert_eq!(teacher_block.created_by_user_id, admin.id());
    assert_eq!(teacher_block.reason, "suspicious reward approvals");
    assert_eq!(
        teacher_block.evidence_reference.as_deref(),
        Some("case://teacher-block")
    );
    assert!(teacher_block.revoked_at.is_none());
    let teacher_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(teacher.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("teacher fraud block notifications should be countable");
    assert_eq!(teacher_notification_count, 1);

    let admin_notifications = notifications::table
        .filter(notifications::user_id.eq(Some(admin.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .load::<Notification>(&mut conn)
        .await
        .expect("platform reviewer fraud block notifications should load");
    assert_eq!(admin_notifications.len(), 1);
    assert!(admin_notifications[0]
        .body
        .contains("suspicious reward approvals"));
    let delegated_platform_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(delegated_platform_auditor.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("delegated platform fraud notifications should be countable");
    assert_eq!(delegated_platform_notification_count, 1);

    let revoked = revoke_reward_fraud_block(&mut conn, admin.id(), teacher_block.id)
        .await
        .expect("admin should revoke teacher reward fraud block");
    assert_eq!(revoked.revoked_by_user_id, Some(admin.id()));
    assert!(revoked.revoked_at.is_some());
    let teacher_revoked_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(teacher.id())))
        .filter(notifications::title.eq("reward_fraud_block:revoked"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("teacher revoked fraud block notifications should be countable");
    assert_eq!(teacher_revoked_notification_count, 1);

    let organization_block = create_reward_fraud_block(
        &mut conn,
        admin.id(),
        RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string(),
            teacher_user_id: None,
            organization_id: Some(organization.id),
            course_id: None,
            reward_policy_id: None,
            reason: "organization reward submissions paused".to_string(),
            evidence_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("admin should block organization reward activity");
    assert_eq!(
        organization_block.scope_type,
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION
    );
    assert_eq!(organization_block.organization_id, Some(organization.id));
    let organization_operator_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(org_operator.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("organization operator fraud block notifications should be countable");
    assert_eq!(organization_operator_notification_count, 1);
    let delegated_organization_operator_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(delegated_org_operator.id())))
        .filter(notifications::title.eq("reward_fraud_block:created"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("delegated organization fraud block notifications should be countable");
    assert_eq!(delegated_organization_operator_notification_count, 1);

    let course_block = create_reward_fraud_block(
        &mut conn,
        admin.id(),
        RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: Some(course.id),
            reward_policy_id: None,
            reason: "course reward rules under review".to_string(),
            evidence_reference: Some("case://course-block".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should block course reward activity through fraud management permission");
    assert_eq!(course_block.scope_type, REWARD_FRAUD_BLOCK_SCOPE_COURSE);
    assert_eq!(course_block.course_id, Some(course.id));

    let stored = reward_fraud_blocks::table
        .find(course_block.id)
        .first::<RewardFraudBlock>(&mut conn)
        .await
        .expect("course reward fraud block should be persisted");
    assert_eq!(stored.created_by_user_id, admin.id());
    assert_eq!(stored.reason, "course reward rules under review");
}
