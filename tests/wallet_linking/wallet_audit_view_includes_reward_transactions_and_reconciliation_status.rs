use crate::support::*;
use crate::wallet_user_audit_helpers::{assert_user_wallet_audit, UserWalletAuditExpectation};

#[actix_web::test]
async fn wallet_audit_view_includes_reward_transactions_and_reconciliation_status() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let student = create_test_user(&mut conn, "wallet_audit_student").await;
    let submitter = create_test_user(&mut conn, "wallet_audit_submitter").await;
    let course_id = create_test_course(&mut conn).await;
    mark_user_kyc_verified(&mut conn, student.id()).await;
    let wallet = link_user_wallet(&mut conn, student.id())
        .await
        .expect("student wallet should link")
        .wallet;

    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id: student.id(),
            submitter_user_id: submitter.id(),
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("wallet_audit_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: REWARD_STATUS_WALLET_CREDITED.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create reward candidate for wallet audit");

    let amount = BigDecimal::from(12);
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::approved_amount.eq(Some(amount.clone())),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter.id())),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to mark reward candidate amount");

    let payout_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_payout"))
        .returning(transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create payout transaction");
    let external_transaction_id: i64 = diesel::insert_into(external_transactions::table)
        .values((
            external_transactions::amount.eq(amount.clone()),
            external_transactions::blockchain_address.eq("0xstudent"),
            external_transactions::chain_id.eq(Some(31337_i64)),
            external_transactions::contract_address.eq(Some("0xcontract")),
            external_transactions::transaction_hash.eq(Some(unique_string("wallet_audit_tx"))),
            external_transactions::log_index.eq(Some(0_i64)),
            external_transactions::event_type.eq(Some("Transfer")),
            external_transactions::from_address.eq(Some("0xtreasury")),
            external_transactions::to_address.eq(Some("0xstudent")),
        ))
        .returning(external_transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create external transaction");
    diesel::insert_into(transactions_external_transactions::table)
        .values((
            transactions_external_transactions::transaction_id.eq(payout_transaction_id),
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to link external transaction");
    let payout_record_id: i64 = diesel::insert_into(reward_payout_records::table)
        .values((
            reward_payout_records::reward_candidate_id.eq(candidate_id),
            reward_payout_records::transaction_id.eq(payout_transaction_id),
            reward_payout_records::external_transaction_id.eq(external_transaction_id),
        ))
        .returning(reward_payout_records::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create reward payout record");

    let wallet_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_wallet_credit"))
        .returning(transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create wallet credit transaction");
    let internal_transaction_id: i64 = diesel::insert_into(internal_transactions::table)
        .values((
            internal_transactions::wallet_id.eq(wallet.id),
            internal_transactions::amount.eq(amount.clone()),
        ))
        .returning(internal_transactions::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create internal transaction");
    diesel::insert_into(transactions_internal_transactions::table)
        .values((
            transactions_internal_transactions::transaction_id.eq(wallet_transaction_id),
            transactions_internal_transactions::internal_transaction_id.eq(internal_transaction_id),
        ))
        .execute(&mut conn)
        .await
        .expect("failed to link internal transaction");
    let credit_record_id: i64 = diesel::insert_into(reward_wallet_credit_records::table)
        .values((
            reward_wallet_credit_records::reward_candidate_id.eq(candidate_id),
            reward_wallet_credit_records::wallet_id.eq(wallet.id),
            reward_wallet_credit_records::transaction_id.eq(wallet_transaction_id),
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transaction_id),
        ))
        .returning(reward_wallet_credit_records::id)
        .get_result(&mut conn)
        .await
        .expect("failed to create reward wallet credit record");
    diesel::update(wallets::table.find(wallet.id))
        .set(wallets::value.eq(amount.clone()))
        .execute(&mut conn)
        .await
        .expect("failed to update wallet balance");
    drop(conn);

    assert_user_wallet_audit(
        &pool,
        UserWalletAuditExpectation {
            student_id: student.id(),
            wallet_id: wallet.id,
            internal_transaction_id,
            wallet_transaction_id,
            external_transaction_id,
            payout_transaction_id,
            candidate_id,
            payout_record_id,
            credit_record_id,
        },
    )
    .await;
}
