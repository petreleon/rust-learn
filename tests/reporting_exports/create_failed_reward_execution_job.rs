async fn create_failed_reward_execution_job(conn: &mut AsyncPgConnection, candidate_id: i64) {
    diesel::insert_into(reward_execution_jobs::table)
        .values((
            reward_execution_jobs::reward_candidate_id.eq(candidate_id),
            reward_execution_jobs::status.eq(RewardExecutionJobStatus::Failed.as_str()),
            reward_execution_jobs::attempts.eq(3),
            reward_execution_jobs::last_error.eq(Some("token transfer failed")),
        ))
        .execute(conn)
        .await
        .expect("failed to create failed reward execution job");
}

async fn mark_reward_approval_decisions(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    teacher_user_id: i32,
    amount_reviewer_user_id: i32,
) {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::teacher_approver_user_id.eq(Some(teacher_user_id)),
            reward_candidates::teacher_decision_reason.eq(Some("course reward approved")),
            reward_candidates::teacher_decided_at.eq(Some(Utc::now())),
            reward_candidates::amount_reviewer_user_id.eq(Some(amount_reviewer_user_id)),
            reward_candidates::approved_amount.eq(Some(BigDecimal::from(10))),
            reward_candidates::amount_decision_reason.eq(Some("platform amount approved")),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
            reward_candidates::updated_at.eq(Utc::now()),
        ))
        .execute(conn)
        .await
        .expect("failed to mark reward approval decisions");
}

async fn create_reward_payout_export_records(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    amount: BigDecimal,
) -> (i64, i64, String) {
    let payout_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_payout"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create payout transaction");
    let transaction_hash = unique_string("platform_export_tx");
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values((
            external_transactions::amount.eq(amount),
            external_transactions::blockchain_address.eq("0xexportstudent"),
            external_transactions::chain_id.eq(Some(31337_i64)),
            external_transactions::contract_address.eq(Some("0xexportcontract")),
            external_transactions::transaction_hash.eq(Some(transaction_hash.clone())),
            external_transactions::log_index.eq(Some(7_i64)),
            external_transactions::event_type.eq(Some("Transfer")),
            external_transactions::from_address.eq(Some("0xexporttreasury")),
            external_transactions::to_address.eq(Some("0xexportstudent")),
        ))
        .returning(external_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create external transaction");
    let payout_record_id = diesel::insert_into(reward_payout_records::table)
        .values((
            reward_payout_records::reward_candidate_id.eq(candidate_id),
            reward_payout_records::transaction_id.eq(payout_transaction_id),
            reward_payout_records::external_transaction_id.eq(external_transaction_id),
        ))
        .returning(reward_payout_records::id)
        .get_result(conn)
        .await
        .expect("failed to create reward payout record");

    (payout_record_id, external_transaction_id, transaction_hash)
}

async fn create_reward_wallet_credit_export_record(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    wallet_id: i32,
    amount: BigDecimal,
) -> i64 {
    let wallet_transaction_id: i64 = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_wallet_credit"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet credit transaction");
    let internal_transaction_id: i64 = diesel::insert_into(internal_transactions::table)
        .values((
            internal_transactions::wallet_id.eq(wallet_id),
            internal_transactions::amount.eq(amount),
        ))
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create internal transaction");

    diesel::insert_into(reward_wallet_credit_records::table)
        .values((
            reward_wallet_credit_records::reward_candidate_id.eq(candidate_id),
            reward_wallet_credit_records::wallet_id.eq(wallet_id),
            reward_wallet_credit_records::transaction_id.eq(wallet_transaction_id),
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transaction_id),
        ))
        .returning(reward_wallet_credit_records::id)
        .get_result(conn)
        .await
        .expect("failed to create reward wallet credit record")
}
