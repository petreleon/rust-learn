async fn credit_reward_wallet_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_credit: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    crate::infra::postgres::rewards::reward_wallet_credit_transaction::credit_reward_wallet_for_candidate(
        conn,
        candidate,
        allow_reconciliation_credit,
        actor_user_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}
