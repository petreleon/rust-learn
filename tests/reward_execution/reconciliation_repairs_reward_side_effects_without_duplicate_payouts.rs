use crate::{
    create_course::*, reconciliation_use_case_helpers::*, support::*, use_case_helpers::*,
};

#[actix_web::test]
async fn reconciliation_repairs_reward_side_effects_without_duplicate_payouts() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("RewardReconciliationCourse")).await;
    let student = create_user_helper(&mut conn, "reward_reconciliation_student").await;
    let submitter = create_user_helper(&mut conn, "reward_reconciliation_submitter").await;
    create_course_reward_policy(&mut conn, course.id, REWARD_PAYMENT_TREASURY_TRANSFER).await;

    let candidate = create_reward_candidate(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_PENDING,
        Some(BigDecimal::from(17)),
    )
    .await;

    let request = RewardTokenConfirmationRequest {
        chain_id: 31337,
        contract_address: "0x00000000000000000000000000000000000000ff".to_string(),
        transaction_hash: unique_hash("reconcile"),
        log_index: 4,
        event_type: "transfer".to_string(),
        from_address: Some("0x00000000000000000000000000000000000000ab".to_string()),
        to_address: "0x00000000000000000000000000000000000000ac".to_string(),
        amount: BigDecimal::from(17),
    };

    let confirmation = record_reward_token_confirmation(&mut conn, candidate.id, request)
        .await
        .expect("token confirmation should create payout evidence");
    diesel::delete(
        transactions_external_transactions::table
            .filter(
                transactions_external_transactions::transaction_id.eq(confirmation.transaction_id),
            )
            .filter(
                transactions_external_transactions::external_transaction_id
                    .eq(confirmation.external_transaction_id),
            ),
    )
    .execute(&mut conn)
    .await
    .expect("external transaction link should be deletable for reconciliation test");

    let first = reconcile_reward_candidate(&mut conn, candidate.id)
        .await
        .expect("reconciliation should repair token-confirmed reward");
    assert!(first.external_transaction_link_repaired);
    assert!(first.wallet_credit_created);
    assert!(first.notification_created);
    assert!(!first.internal_transaction_link_repaired);
    assert_eq!(first.final_status.as_str(), REWARD_STATUS_NOTIFIED);

    let wallet = wallets::table
        .filter(wallets::user_id.eq(Some(student.id())))
        .filter(wallets::organization_id.is_null())
        .first::<rust_learn::models::wallet::Wallet>(&mut conn)
        .await
        .expect("wallet should exist after reconciliation");
    assert_eq!(wallet.value, BigDecimal::from(17));

    let external_link_count = transactions_external_transactions::table
        .filter(transactions_external_transactions::transaction_id.eq(confirmation.transaction_id))
        .filter(
            transactions_external_transactions::external_transaction_id
                .eq(confirmation.external_transaction_id),
        )
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("external link should be countable after reconciliation");
    assert_eq!(external_link_count, 1);

    let credit_record = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(candidate.id))
        .first::<rust_learn::models::reward_wallet_credit_record::RewardWalletCreditRecord>(
            &mut conn,
        )
        .await
        .expect("reconciliation should create reward wallet credit record");
    assert!(credit_record.notification_id.is_some());

    let notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(student.id())))
        .filter(notifications::title.eq("reward:wallet_credited"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("reward notifications should be countable");
    assert_eq!(notification_count, 1);

    diesel::delete(
        transactions_internal_transactions::table
            .filter(
                transactions_internal_transactions::transaction_id.eq(credit_record.transaction_id),
            )
            .filter(
                transactions_internal_transactions::internal_transaction_id
                    .eq(credit_record.internal_transaction_id),
            ),
    )
    .execute(&mut conn)
    .await
    .expect("internal transaction link should be deletable for reconciliation test");

    let second = reconcile_reward_candidate(&mut conn, candidate.id)
        .await
        .expect("reconciliation should repair missing internal link");
    assert!(!second.external_transaction_link_repaired);
    assert!(!second.wallet_credit_created);
    assert!(!second.notification_created);
    assert!(second.internal_transaction_link_repaired);
    assert_eq!(second.final_status.as_str(), REWARD_STATUS_NOTIFIED);

    let wallet_after_second_reconcile = wallets::table
        .find(wallet.id)
        .select(wallets::value)
        .first::<BigDecimal>(&mut conn)
        .await
        .expect("wallet should still exist after second reconciliation");
    assert_eq!(wallet_after_second_reconcile, BigDecimal::from(17));

    let notification_count_after_second_reconcile = notifications::table
        .filter(notifications::user_id.eq(Some(student.id())))
        .filter(notifications::title.eq("reward:wallet_credited"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("reward notifications should still be countable");
    assert_eq!(notification_count_after_second_reconcile, 1);

    let third = reconcile_reward_candidate(&mut conn, candidate.id)
        .await
        .expect("fully repaired reward should reconcile idempotently");
    assert!(!third.external_transaction_link_repaired);
    assert!(!third.wallet_credit_created);
    assert!(!third.notification_created);
    assert!(!third.internal_transaction_link_repaired);
    assert_eq!(third.final_status.as_str(), REWARD_STATUS_NOTIFIED);
}
