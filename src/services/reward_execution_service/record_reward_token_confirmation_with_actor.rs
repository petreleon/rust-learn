pub async fn record_reward_token_confirmation(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    let mut store = PostgresRewardTokenConfirmationStore::new(conn);
    crate::application::rewards::record_token_confirmation::record_reward_token_confirmation(
        &mut store,
        candidate_id,
        request,
    )
    .await
    .map_err(RewardExecutionError::from)
}

pub async fn record_reward_token_confirmation_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    let mut store = PostgresRewardTokenConfirmationStore::new(conn);
    crate::application::rewards::record_token_confirmation::record_reward_token_confirmation_for_actor(
        &mut store,
        actor_user_id,
        candidate_id,
        request,
    )
    .await
    .map_err(RewardExecutionError::from)
}

#[cfg(test)]
#[allow(dead_code)]
fn ensure_candidate_ready_for_payout(
    candidate: &RewardCandidate,
) -> Result<(), RewardExecutionError> {
    if candidate.status == REWARD_STATUS_AMOUNT_APPROVED {
        Ok(())
    } else {
        Err(RewardExecutionError::InvalidStatus(
            "reward candidate must be amount approved before payout planning".to_string(),
        ))
    }
}
