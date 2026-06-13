pub async fn plan_reward_payout(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    let mut store = PostgresRewardPayoutPlanStore::new(conn);
    crate::application::rewards::plan_payout::plan_reward_payout(&mut store, candidate_id)
        .await
        .map_err(RewardExecutionError::from)
}

pub async fn plan_reward_payout_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    let mut store = PostgresRewardPayoutPlanStore::new(conn);
    crate::application::rewards::plan_payout::plan_reward_payout_for_actor(
        &mut store,
        actor_user_id,
        candidate_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}

pub async fn credit_reward_wallet(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    let mut store = PostgresRewardWalletCreditStore::new(conn);
    crate::application::rewards::credit_wallet::credit_reward_wallet(&mut store, candidate_id)
        .await
        .map_err(RewardExecutionError::from)
}

pub async fn credit_reward_wallet_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    let mut store = PostgresRewardWalletCreditStore::new(conn);
    crate::application::rewards::credit_wallet::credit_reward_wallet_for_actor(
        &mut store,
        actor_user_id,
        candidate_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}

pub async fn notify_reward_wallet_credit(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    let mut store = PostgresRewardWalletCreditNotificationStore::new(conn);
    crate::application::rewards::notify_wallet_credit::notify_reward_wallet_credit(
        &mut store,
        candidate_id,
    )
    .await
    .map_err(RewardExecutionError::from)
}
