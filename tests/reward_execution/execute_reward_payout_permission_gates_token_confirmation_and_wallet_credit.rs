use crate::{create_course::*, support::*, use_case_helpers::*};

#[actix_web::test]
async fn execute_reward_payout_permission_gates_token_confirmation_and_wallet_credit() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PayoutPermissionCourse")).await;
    let student = create_user_helper(&mut conn, "payout_permission_student").await;
    let submitter = create_user_helper(&mut conn, "payout_permission_submitter").await;
    let executor = create_user_helper(&mut conn, "payout_permission_executor").await;
    let no_permission_user = create_user_helper(&mut conn, "payout_permission_denied").await;
    let executor_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("REWARD_PAYOUT_EXECUTOR"),
        &[Permissions::EXECUTE_REWARD_PAYOUT],
    )
    .await;
    assign_platform_role_id(&mut conn, executor.id(), executor_role_id).await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_PENDING,
        Some(BigDecimal::from(19)),
    )
    .await;
    let request = RewardTokenConfirmationRequest {
        chain_id: 31337,
        contract_address: "0x0000000000000000000000000000000000000101".to_string(),
        transaction_hash: unique_hash("permission"),
        log_index: 5,
        event_type: RewardTokenEventType::Transfer,
        from_address: Some("0x0000000000000000000000000000000000000102".to_string()),
        to_address: "0x0000000000000000000000000000000000000103".to_string(),
        amount: BigDecimal::from(19),
    };

    let denied_confirmation = record_reward_token_confirmation_for_actor(
        &mut conn,
        no_permission_user.id(),
        candidate.id,
        request.clone(),
    )
    .await
    .expect_err("token confirmation should require reward payout execution permission");
    assert!(matches!(
        denied_confirmation,
        RewardExecutionError::PermissionDenied(permission)
            if permission == Permissions::EXECUTE_REWARD_PAYOUT.to_string()
    ));

    let status_after_denied_confirmation = reward_candidates::table
        .find(candidate.id)
        .select(reward_candidates::status)
        .first::<String>(&mut conn)
        .await
        .expect("candidate status should be queryable");
    assert_eq!(
        status_after_denied_confirmation,
        REWARD_STATUS_TOKEN_PENDING
    );

    record_reward_token_confirmation_for_actor(&mut conn, executor.id(), candidate.id, request)
        .await
        .expect("payout executor should record token confirmation");

    let denied_wallet_credit =
        credit_reward_wallet_for_actor(&mut conn, no_permission_user.id(), candidate.id)
            .await
            .expect_err("wallet credit should require reward payout execution permission");
    assert!(matches!(
        denied_wallet_credit,
        RewardExecutionError::PermissionDenied(permission)
            if permission == Permissions::EXECUTE_REWARD_PAYOUT.to_string()
    ));

    let credit_record_count_after_denied: i64 = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(candidate.id))
        .count()
        .get_result(&mut conn)
        .await
        .expect("wallet credit records should be countable");
    assert_eq!(credit_record_count_after_denied, 0);

    let credited = credit_reward_wallet_for_actor(&mut conn, executor.id(), candidate.id)
        .await
        .expect("payout executor should credit the reward wallet");
    assert!(credited.credited);
    assert_eq!(credited.amount, BigDecimal::from(19));

    let audit = list_reward_audit_events(&mut conn, candidate.id)
        .await
        .expect("reward execution audit events should load");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].event_type, REWARD_AUDIT_EVENT_TOKEN_CONFIRMED);
    assert_eq!(audit[0].actor_user_id, Some(executor.id()));
    assert_eq!(audit[1].event_type, REWARD_AUDIT_EVENT_WALLET_CREDITED);
    assert_eq!(audit[1].actor_user_id, Some(executor.id()));
}
