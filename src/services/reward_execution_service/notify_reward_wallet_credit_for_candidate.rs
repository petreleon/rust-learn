async fn notify_reward_wallet_credit_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_repair: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    crate::infra::postgres::rewards::reward_wallet_credit_notification_transaction::notify_reward_wallet_credit_for_candidate(
        conn,
        candidate,
        allow_reconciliation_repair,
        actor_user_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}

fn ensure_candidate_reconcilable(candidate: &RewardCandidate) -> Result<(), RewardExecutionError> {
    if [
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
    {
        Ok(())
    } else {
        Err(RewardExecutionError::InvalidStatus(
            "reward candidate has no confirmed state to reconcile".to_string(),
        ))
    }
}

fn should_create_reconciliation_wallet_credit(candidate: &RewardCandidate) -> bool {
    [
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
}
