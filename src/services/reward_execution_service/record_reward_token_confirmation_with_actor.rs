async fn record_reward_token_confirmation_with_actor(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
    actor_user_id: Option<i32>,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    validate_token_confirmation_request(&request)?;

    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
            Box::pin(async move {
                if let Some(existing_record) =
                    reward_payout_record_repository::find_reward_payout_record_by_candidate(
                        conn,
                        candidate_id,
                    )
                    .await?
                {
                    return Ok(RewardTokenConfirmationResult {
                        candidate_id,
                        transaction_id: existing_record.transaction_id,
                        external_transaction_id: existing_record.external_transaction_id,
                        payout_record_id: existing_record.id,
                        inserted_external_transaction: false,
                    });
                }

                let candidate =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                if candidate.status != REWARD_STATUS_TOKEN_PENDING {
                    return Err(RewardExecutionError::InvalidStatus(
                        "reward candidate must be token pending before token confirmation"
                            .to_string(),
                    ));
                }

                let (transaction_id, external_transaction_id, inserted_external_transaction) =
                    record_external_reward_transaction(conn, &request).await?;
                let payout_record = reward_payout_record_repository::create_reward_payout_record(
                    conn,
                    NewRewardPayoutRecord {
                        reward_candidate_id: candidate.id,
                        transaction_id,
                        external_transaction_id,
                    },
                )
                .await?;
                let updated = mark_candidate_token_confirmed(conn, candidate.id).await?;
                reward_audit_event_repository::create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: updated.id,
                        actor_user_id,
                        event_type: REWARD_AUDIT_EVENT_TOKEN_CONFIRMED.to_string(),
                        from_status: Some(candidate.status.clone()),
                        to_status: updated.status,
                        reason: None,
                        metadata: serde_json::json!({
                            "transaction_id": transaction_id,
                            "external_transaction_id": external_transaction_id,
                            "payout_record_id": payout_record.id,
                            "inserted_external_transaction": inserted_external_transaction,
                        }),
                    },
                )
                .await?;

                Ok(RewardTokenConfirmationResult {
                    candidate_id: candidate.id,
                    transaction_id,
                    external_transaction_id,
                    payout_record_id: payout_record.id,
                    inserted_external_transaction,
                })
            })
        })
        .await?;

    log::info!(
        "event=reward_token_confirmed candidate_id={} transaction_id={} external_transaction_id={} payout_record_id={} inserted_external_transaction={}",
        result.candidate_id,
        result.transaction_id,
        result.external_transaction_id,
        result.payout_record_id,
        result.inserted_external_transaction
    );

    Ok(result)
}

async fn ensure_can_execute_reward_payout(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<(), RewardExecutionError> {
    let permission = Permissions::EXECUTE_REWARD_PAYOUT.to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission).await? {
        Ok(())
    } else {
        Err(RewardExecutionError::PermissionDenied(permission))
    }
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

fn approved_positive_amount(
    candidate: &RewardCandidate,
) -> Result<BigDecimal, RewardExecutionError> {
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
    Ok(amount)
}
