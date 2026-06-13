pub async fn notify_reward_wallet_credit_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    let mut store = PostgresRewardWalletCreditNotificationStore::new(conn);
    crate::application::rewards::notify_wallet_credit::notify_reward_wallet_credit_for_actor(
        &mut store,
        actor_user_id,
        candidate_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}

pub async fn reconcile_reward_candidate(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    reconcile_reward_candidate_with_actor(conn, candidate_id, None).await
}

pub async fn reconcile_reward_candidate_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    reconcile_reward_candidate_with_actor(conn, candidate_id, Some(actor_user_id)).await
}
