#[actix_web::test]
async fn reward_compensation_records_adjust_wallet_without_mutating_candidate_decision() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("CompensationCourse")).await;
    let admin = create_user_helper(&mut conn, "compensation_admin").await;
    let stranger = create_user_helper(&mut conn, "compensation_stranger").await;
    let student = create_user_helper(&mut conn, "compensation_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    let candidate_id =
        create_completed_reward_candidate(&mut conn, course.id, student.id(), admin.id()).await;

    let denied = record_reward_compensation(
        &mut conn,
        stranger.id(),
        RewardCompensationRequest {
            reward_candidate_id: candidate_id,
            amount: BigDecimal::from(5),
            reason: "manual make-good".to_string(),
            idempotency_key: unique_string("compensation_denied"),
        },
    )
    .await
    .expect_err("user without platform wallet permission should not compensate rewards");
    assert!(matches!(
        denied,
        RewardCompensationError::PermissionDenied(_)
    ));

    let idempotency_key = unique_string("compensation_create");
    let compensation = record_reward_compensation(
        &mut conn,
        admin.id(),
        RewardCompensationRequest {
            reward_candidate_id: candidate_id,
            amount: BigDecimal::from(5),
            reason: "manual make-good".to_string(),
            idempotency_key: idempotency_key.clone(),
        },
    )
    .await
    .expect("admin should record reward compensation");
    assert!(compensation.created);
    assert_eq!(compensation.record.reward_candidate_id, candidate_id);
    assert_eq!(compensation.record.amount, BigDecimal::from(5));
    assert_eq!(compensation.record.reason, "manual make-good");
    assert_eq!(compensation.wallet.value, BigDecimal::from(5));

    let duplicate = record_reward_compensation(
        &mut conn,
        admin.id(),
        RewardCompensationRequest {
            reward_candidate_id: candidate_id,
            amount: BigDecimal::from(5),
            reason: "manual make-good".to_string(),
            idempotency_key,
        },
    )
    .await
    .expect("idempotent compensation retry should succeed");
    assert!(!duplicate.created);
    assert_eq!(duplicate.record.id, compensation.record.id);
    assert_eq!(duplicate.wallet.value, BigDecimal::from(5));

    let transaction_type = transactions::table
        .find(compensation.record.transaction_id)
        .select(transactions::type_)
        .first::<String>(&mut conn)
        .await
        .expect("compensation transaction should exist");
    assert_eq!(transaction_type, REWARD_TRANSACTION_TYPE_COMPENSATION);

    let wallet_value = wallets::table
        .find(compensation.wallet.id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should remain queryable");
    assert_eq!(wallet_value, BigDecimal::from(5));

    let candidate_after_compensation = find_candidate(&mut conn, candidate_id)
        .await
        .expect("candidate should remain queryable");
    assert_eq!(candidate_after_compensation.status, REWARD_STATUS_COMPLETED);
    assert_eq!(
        candidate_after_compensation.approved_amount,
        Some(BigDecimal::from(10))
    );
}
