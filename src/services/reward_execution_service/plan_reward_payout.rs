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
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                credit_reward_wallet_for_candidate(conn, &candidate, false, None).await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit candidate_id={} wallet_id={} amount={} credited={} credit_record_id={:?} transaction_id={:?} internal_transaction_id={:?}",
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.credited,
        result.credit_record_id,
        result.transaction_id,
        result.internal_transaction_id
    );

    Ok(result)
}

pub async fn credit_reward_wallet_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                credit_reward_wallet_for_candidate(conn, &candidate, false, Some(actor_user_id))
                    .await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit actor_user_id={} candidate_id={} wallet_id={} amount={} credited={} credit_record_id={:?} transaction_id={:?} internal_transaction_id={:?}",
        actor_user_id,
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.credited,
        result.credit_record_id,
        result.transaction_id,
        result.internal_transaction_id
    );

    Ok(result)
}

pub async fn notify_reward_wallet_credit(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                notify_reward_wallet_credit_for_candidate(conn, &candidate, false, None).await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit_notification candidate_id={} wallet_id={} amount={} notified={} notification_id={:?} transaction_id={}",
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.notified,
        result.notification_id,
        result.transaction_id
    );

    Ok(result)
}
