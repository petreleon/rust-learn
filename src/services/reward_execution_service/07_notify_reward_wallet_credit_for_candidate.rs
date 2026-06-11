async fn notify_reward_wallet_credit_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_repair: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditNotificationResult, RewardExecutionError> {
    let allowed_to_inspect_notification = [
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
        || (allow_reconciliation_repair
            && [
                REWARD_STATUS_AMOUNT_APPROVED,
                REWARD_STATUS_TOKEN_CONFIRMED,
                REWARD_STATUS_NEEDS_RECONCILIATION,
            ]
            .contains(&candidate.status.as_str()));

    if !allowed_to_inspect_notification {
        return Err(RewardExecutionError::InvalidStatus(
            "reward candidate must be wallet credited before notification".to_string(),
        ));
    }

    let amount = approved_positive_amount(candidate)?;
    let credit_record =
        reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
            conn,
            candidate.id,
        )
        .await?
        .ok_or_else(|| {
            RewardExecutionError::InvalidStatus(
                "wallet credited candidate is missing a reward wallet credit record".to_string(),
            )
        })?;

    if let Some(notification_id) = credit_record.notification_id {
        return Ok(RewardWalletCreditNotificationResult {
            candidate_id: candidate.id,
            wallet_id: credit_record.wallet_id,
            notification_id: Some(notification_id),
            transaction_id: credit_record.transaction_id,
            amount,
            notified: false,
        });
    }

    let can_create_missing_notification = candidate.status == REWARD_STATUS_WALLET_CREDITED
        || (allow_reconciliation_repair
            && [
                REWARD_STATUS_AMOUNT_APPROVED,
                REWARD_STATUS_TOKEN_CONFIRMED,
                REWARD_STATUS_NEEDS_RECONCILIATION,
            ]
            .contains(&candidate.status.as_str()));

    if !can_create_missing_notification {
        return Err(RewardExecutionError::InvalidStatus(
            "notified candidate is missing its notification reference".to_string(),
        ));
    }

    let course_title = courses::table
        .find(candidate.course_id)
        .select(courses::title)
        .first::<String>(conn)
        .await?;
    let message = reward_wallet_credit_notification(
        candidate.course_id,
        course_title,
        amount.to_string(),
        credit_record.wallet_id,
        credit_record.transaction_id,
    );
    let notification_id =
        create_notification(conn, candidate.student_user_id, message.title, message.body)
            .await
            .map_err(|e| RewardExecutionError::Database(e.to_string()))?;
    reward_wallet_credit_record_repository::mark_reward_wallet_credit_record_notified(
        conn,
        credit_record.id,
        notification_id,
    )
    .await?;
    let updated = mark_candidate_notified(conn, candidate.id).await?;
    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id,
            event_type: REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED.to_string(),
            from_status: Some(candidate.status.clone()),
            to_status: updated.status,
            reason: None,
            metadata: serde_json::json!({
                "wallet_id": credit_record.wallet_id,
                "notification_id": notification_id,
                "transaction_id": credit_record.transaction_id,
            }),
        },
    )
    .await?;

    Ok(RewardWalletCreditNotificationResult {
        candidate_id: candidate.id,
        wallet_id: credit_record.wallet_id,
        notification_id: Some(notification_id),
        transaction_id: credit_record.transaction_id,
        amount,
        notified: true,
    })
}

fn ensure_candidate_reconcilable(candidate: &RewardCandidate) -> Result<(), RewardExecutionError> {
    if [
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
    {
        Ok(())
    } else {
        Err(RewardExecutionError::InvalidStatus(
            "reward candidate has no confirmed state to reconcile".to_string(),
        ))
    }
}

fn should_create_reconciliation_wallet_credit(candidate: &RewardCandidate) -> bool {
    [
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ]
    .contains(&candidate.status.as_str())
}
