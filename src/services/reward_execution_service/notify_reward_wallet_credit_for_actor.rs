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
    let mut store = PostgresRewardReconciliationStore::new(conn);
    crate::application::rewards::reconcile_candidate::reconcile_reward_candidate(
        &mut store,
        candidate_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}

pub async fn reconcile_reward_candidate_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    let mut store = PostgresRewardReconciliationStore::new(conn);
    crate::application::rewards::reconcile_candidate::reconcile_reward_candidate_for_actor(
        &mut store,
        actor_user_id,
        candidate_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}
