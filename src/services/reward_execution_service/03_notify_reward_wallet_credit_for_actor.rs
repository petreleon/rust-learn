pub async fn notify_reward_wallet_credit_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                notify_reward_wallet_credit_for_candidate(
                    conn,
                    &candidate,
                    false,
                    Some(actor_user_id),
                )
                .await
            })
        })
        .await?;

    log::info!(
        "event=reward_wallet_credit_notification actor_user_id={} candidate_id={} wallet_id={} amount={} notified={} notification_id={:?} transaction_id={}",
        actor_user_id,
        result.candidate_id,
        result.wallet_id,
        result.amount,
        result.notified,
        result.notification_id,
        result.transaction_id
    );

    Ok(result)
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
