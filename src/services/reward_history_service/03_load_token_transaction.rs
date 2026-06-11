async fn load_token_transaction(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> Result<Option<StudentRewardTokenTransaction>, StudentRewardHistoryError> {
    type TokenTransactionRow = (
        i64,
        i64,
        i64,
        DateTime<Utc>,
        BigDecimal,
        String,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<String>,
    );

    let row = reward_payout_records::table
        .inner_join(
            external_transactions::table
                .on(reward_payout_records::external_transaction_id.eq(external_transactions::id)),
        )
        .filter(reward_payout_records::reward_candidate_id.eq(reward_candidate_id))
        .select((
            reward_payout_records::id,
            reward_payout_records::transaction_id,
            reward_payout_records::external_transaction_id,
            reward_payout_records::created_at,
            external_transactions::amount,
            external_transactions::blockchain_address,
            external_transactions::chain_id,
            external_transactions::contract_address,
            external_transactions::transaction_hash,
            external_transactions::log_index,
            external_transactions::event_type,
            external_transactions::from_address,
            external_transactions::to_address,
        ))
        .first::<TokenTransactionRow>(conn)
        .await
        .optional()?;

    Ok(row.map(
        |(
            reward_payout_record_id,
            payout_transaction_id,
            external_transaction_id,
            recorded_at,
            amount,
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
        )| StudentRewardTokenTransaction {
            reward_payout_record_id,
            payout_transaction_id,
            external_transaction_id,
            amount: amount.to_string(),
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
            recorded_at,
        },
    ))
}

fn normalize_reward_status(status: &str) -> Result<String, StudentRewardHistoryError> {
    let normalized = status.trim().to_ascii_lowercase().replace(['-', ' '], "_");
    match normalized.as_str() {
        REWARD_STATUS_PENDING_TEACHER_APPROVAL
        | REWARD_STATUS_TEACHER_APPROVED
        | REWARD_STATUS_TEACHER_REJECTED
        | REWARD_STATUS_AMOUNT_APPROVED
        | REWARD_STATUS_AMOUNT_REJECTED
        | REWARD_STATUS_ADJUSTED
        | REWARD_STATUS_TOKEN_PENDING
        | REWARD_STATUS_TOKEN_CONFIRMED
        | REWARD_STATUS_WALLET_CREDITED
        | REWARD_STATUS_NOTIFIED
        | REWARD_STATUS_COMPLETED
        | REWARD_STATUS_NEEDS_RECONCILIATION
        | REWARD_STATUS_FAILED => Ok(normalized),
        _ => Err(StudentRewardHistoryError::InvalidInput(
            "unsupported reward candidate status".to_string(),
        )),
    }
}
