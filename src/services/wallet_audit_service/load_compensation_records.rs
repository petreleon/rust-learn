async fn load_compensation_records(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> QueryResult<Vec<WalletCompensationRecordAudit>> {
    let records = reward_compensation_records::table
        .filter(reward_compensation_records::wallet_id.eq(wallet_id))
        .order(reward_compensation_records::created_at.desc())
        .load::<RewardCompensationRecord>(conn)
        .await?;

    Ok(records
        .into_iter()
        .map(|record| WalletCompensationRecordAudit {
            id: record.id,
            reward_candidate_id: record.reward_candidate_id,
            wallet_id: record.wallet_id,
            transaction_id: record.transaction_id,
            internal_transaction_id: record.internal_transaction_id,
            amount: record.amount.to_string(),
            reason: record.reason,
            idempotency_key: record.idempotency_key,
            created_by_user_id: record.created_by_user_id,
            created_at: record.created_at,
        })
        .collect())
}

fn reward_reconciliation_status(
    candidate: &RewardCandidate,
    credit_record: Option<&RewardWalletCreditRecord>,
    payout_record: Option<&RewardPayoutRecord>,
) -> String {
    let status = match candidate.status.as_str() {
        REWARD_STATUS_NEEDS_RECONCILIATION => "needs_reconciliation",
        REWARD_STATUS_TOKEN_CONFIRMED if payout_record.is_none() => "needs_payout_record",
        REWARD_STATUS_TOKEN_CONFIRMED if credit_record.is_none() => "needs_wallet_credit",
        REWARD_STATUS_WALLET_CREDITED if credit_record.is_none() => "needs_wallet_credit_record",
        REWARD_STATUS_WALLET_CREDITED => "needs_notification",
        REWARD_STATUS_NOTIFIED | REWARD_STATUS_COMPLETED
            if credit_record
                .and_then(|record| record.notification_id)
                .is_none() =>
        {
            "needs_notification_record"
        }
        REWARD_STATUS_NOTIFIED | REWARD_STATUS_COMPLETED => "reconciled",
        REWARD_STATUS_AMOUNT_APPROVED | REWARD_STATUS_TOKEN_PENDING => "pending_execution",
        REWARD_STATUS_AMOUNT_REJECTED | REWARD_STATUS_TEACHER_REJECTED | REWARD_STATUS_FAILED => {
            "closed_without_payout"
        }
        REWARD_STATUS_PENDING_TEACHER_APPROVAL | REWARD_STATUS_TEACHER_APPROVED => {
            "pending_decision"
        }
        _ if credit_record.is_some() => "reconciled",
        _ => "pending",
    };

    status.to_string()
}

#[cfg(test)]
mod tests;
