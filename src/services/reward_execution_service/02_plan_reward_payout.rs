pub async fn plan_reward_payout(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    let candidate = reward_candidate_repository::find_candidate(conn, candidate_id).await?;
    ensure_candidate_ready_for_payout(&candidate)?;

    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardExecutionError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardExecutionError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }

    let policy = resolve_active_reward_policy(conn, candidate.course_id, &candidate.event_type)
        .await?
        .ok_or(RewardExecutionError::NoActivePolicy)?;
    let payout_method = select_payout_method(conn, &policy).await?;
    let requires_token_confirmation = payout_method != REWARD_PAYOUT_METHOD_OFF_CHAIN;

    let plan = RewardPayoutPlan {
        candidate_id: candidate.id,
        policy_id: policy.id,
        amount,
        payment_strategy: policy.payment_strategy,
        payout_method,
        requires_token_confirmation,
    };

    log::info!(
        "event=reward_payout_planned candidate_id={} policy_id={} amount={} payment_strategy={} payout_method={} requires_token_confirmation={}",
        plan.candidate_id,
        plan.policy_id,
        plan.amount,
        plan.payment_strategy,
        plan.payout_method,
        plan.requires_token_confirmation
    );

    Ok(plan)
}

pub async fn plan_reward_payout_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    ensure_can_execute_reward_payout(conn, actor_user_id).await?;
    plan_reward_payout(conn, candidate_id).await
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
