async fn credit_reward_wallet_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_credit: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    let amount = approved_positive_amount(candidate)?;
    let credit_record =
        reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
            conn,
            candidate.id,
        )
        .await?;

    if let Some(record) = credit_record.as_ref() {
        return Ok(RewardWalletCreditResult {
            candidate_id: candidate.id,
            wallet_id: record.wallet_id,
            credit_record_id: Some(record.id),
            transaction_id: Some(record.transaction_id),
            internal_transaction_id: Some(record.internal_transaction_id),
            amount,
            credited: false,
        });
    }

    if [
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
    ]
    .contains(&candidate.status.as_str())
    {
        return Err(RewardExecutionError::InvalidStatus(
            "wallet credited candidate is missing a reward wallet credit record".to_string(),
        ));
    }

    if candidate.status == REWARD_STATUS_NEEDS_RECONCILIATION {
        if !allow_reconciliation_credit {
            return Err(RewardExecutionError::InvalidStatus(
                "needs reconciliation candidate requires confirmed payout evidence before wallet credit"
                    .to_string(),
            ));
        }
    } else {
        ensure_wallet_credit_allowed(conn, candidate).await?;
    }

    let wallet = wallet_service::link_user_wallet(conn, candidate.student_user_id)
        .await?
        .wallet;
    let wallet = credit_wallet_balance(conn, wallet.id, amount.clone()).await?;
    let internal_transaction_id =
        create_internal_transaction(conn, wallet.id, amount.clone()).await?;
    let transaction_id = create_wallet_credit_transaction(conn, internal_transaction_id).await?;
    let credit_record = reward_wallet_credit_record_repository::create_reward_wallet_credit_record(
        conn,
        NewRewardWalletCreditRecord {
            reward_candidate_id: candidate.id,
            wallet_id: wallet.id,
            transaction_id,
            internal_transaction_id,
        },
    )
    .await?;
    let updated = mark_candidate_wallet_credited(conn, candidate.id).await?;
    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id,
            event_type: REWARD_AUDIT_EVENT_WALLET_CREDITED.to_string(),
            from_status: Some(candidate.status.clone()),
            to_status: updated.status,
            reason: None,
            metadata: serde_json::json!({
                "wallet_id": wallet.id,
                "credit_record_id": credit_record.id,
                "transaction_id": transaction_id,
                "internal_transaction_id": internal_transaction_id,
            }),
        },
    )
    .await?;

    Ok(RewardWalletCreditResult {
        candidate_id: candidate.id,
        wallet_id: wallet.id,
        credit_record_id: Some(credit_record.id),
        transaction_id: Some(transaction_id),
        internal_transaction_id: Some(internal_transaction_id),
        amount,
        credited: true,
    })
}
