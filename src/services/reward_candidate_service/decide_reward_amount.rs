pub async fn decide_reward_amount(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
    request: RewardAmountDecisionRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    let target_status = normalize_amount_decision_status(&request.status)?;
    ensure_platform_permission(conn, actor_user_id, Permissions::APPROVE_REWARD_AMOUNT).await?;

    let approved_amount = match target_status.as_str() {
        REWARD_STATUS_AMOUNT_APPROVED => {
            let amount = request.approved_amount.ok_or_else(|| {
                RewardCandidateError::InvalidInput(
                    "approved amount is required for amount approval".to_string(),
                )
            })?;
            if amount < BigDecimal::from(0) {
                return Err(RewardCandidateError::InvalidInput(
                    "approved amount cannot be negative".to_string(),
                ));
            }
            Some(amount)
        }
        REWARD_STATUS_AMOUNT_REJECTED => None,
        _ => unreachable!("amount status normalization returned unsupported status"),
    };

    let updated = conn
        .transaction::<_, RewardCandidateError, _>(|conn| {
            Box::pin(async move {
                let existing =
                    reward_candidate_repository::find_candidate(conn, candidate_id).await?;
                if existing.status == target_status {
                    return Ok(existing);
                }

                if existing.status != REWARD_STATUS_TEACHER_APPROVED {
                    return Err(RewardCandidateError::InvalidStatus(
                        "reward amount can be decided only after teacher approval".to_string(),
                    ));
                }
                let from_status = existing.status.clone();

                let teacher_user_ids = candidate_teacher_user_ids(&existing);
                ensure_no_active_reward_fraud_block(
                    conn,
                    teacher_user_ids.as_slice(),
                    existing.course_id,
                    &existing.event_type,
                    existing.source_organization_id,
                )
                .await?;

                let updated = reward_candidate_repository::update_amount_decision(
                    conn,
                    candidate_id,
                    actor_user_id,
                    &target_status,
                    approved_amount.clone(),
                    request.decision_reason.as_deref(),
                    Utc::now(),
                )
                .await
                .map_err(RewardCandidateError::from)?;

                reward_audit_event_repository::create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: updated.id,
                        actor_user_id: Some(actor_user_id),
                        event_type: REWARD_AUDIT_EVENT_AMOUNT_DECISION.to_string(),
                        from_status: Some(from_status),
                        to_status: updated.status.clone(),
                        reason: request.decision_reason.clone(),
                        metadata: json!({
                            "approved_amount": updated.approved_amount.as_ref().map(ToString::to_string),
                        }),
                    },
                )
                .await
                .map_err(RewardCandidateError::from)?;

                if target_status == REWARD_STATUS_AMOUNT_APPROVED {
                    reward_execution_job_repository::enqueue_reward_execution_job(
                        conn,
                        candidate_id,
                    )
                    .await?;
                }

                Ok(updated)
            })
        })
        .await?;

    let approved_amount = updated
        .approved_amount
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "none".to_string());
    log::info!(
        "event=reward_candidate_amount_decision candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} approved_amount={} event_type={} source_scope={} source_organization_id={:?}",
        updated.id,
        actor_user_id,
        updated.student_user_id,
        updated.course_id,
        updated.status,
        approved_amount,
        updated.event_type,
        updated.source_scope,
        updated.source_organization_id
    );

    Ok(updated)
}
