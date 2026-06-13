async fn reconcile_reward_candidate_with_actor(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    actor_user_id: Option<i32>,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    let result = conn
        .transaction::<_, RewardExecutionError, _>(|conn| {
        Box::pin(async move {
            let mut candidate =
                reward_candidate_repository::find_candidate(conn, candidate_id).await?;
            ensure_candidate_reconcilable(&candidate)?;
            let initial_status = candidate.status.clone();

            let payout_record =
                reward_payout_record_repository::find_reward_payout_record_by_candidate(
                    conn,
                    candidate.id,
                )
                .await?;
            let external_transaction_link_repaired = match payout_record.as_ref() {
                Some(record) => {
                    ensure_external_transaction_link(
                        conn,
                        record.transaction_id,
                        record.external_transaction_id,
                    )
                    .await?
                }
                None => false,
            };

            let mut credit_record =
                reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
                    conn,
                    candidate.id,
                )
                .await?;
            let mut wallet_credit_created = false;
            if credit_record.is_none() && should_create_reconciliation_wallet_credit(&candidate) {
                wallet_credit_created = credit_reward_wallet_for_candidate(
                    conn,
                    &candidate,
                    payout_record.is_some(),
                    actor_user_id,
                )
                .await?
                .credited;
                candidate = reward_candidate_repository::find_candidate(conn, candidate.id).await?;
                credit_record =
                    reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
                        conn,
                        candidate.id,
                    )
                    .await?;
            }

            let internal_transaction_link_repaired = match credit_record.as_ref() {
                Some(record) => {
                    ensure_internal_transaction_link(
                        conn,
                        record.transaction_id,
                        record.internal_transaction_id,
                    )
                    .await?
                }
                None => false,
            };

            let mut notification_created = false;
            if credit_record.is_some() {
                let notification_result =
                    notify_reward_wallet_credit_for_candidate(
                        conn,
                        &candidate,
                        true,
                        actor_user_id,
                    )
                    .await?;
                notification_created = notification_result.notified;
                candidate = reward_candidate_repository::find_candidate(conn, candidate.id).await?;
            }

            if wallet_credit_created
                || notification_created
                || external_transaction_link_repaired
                || internal_transaction_link_repaired
            {
                reward_audit_event_repository::create_reward_audit_event(
                    conn,
                    NewRewardAuditEvent {
                        reward_candidate_id: candidate.id,
                        actor_user_id,
                        event_type: REWARD_AUDIT_EVENT_RECONCILED.to_string(),
                        from_status: Some(initial_status),
                        to_status: candidate.status.clone(),
                        reason: None,
                        metadata: serde_json::json!({
                            "wallet_credit_created": wallet_credit_created,
                            "notification_created": notification_created,
                            "external_transaction_link_repaired": external_transaction_link_repaired,
                            "internal_transaction_link_repaired": internal_transaction_link_repaired,
                        }),
                    },
                )
                .await?;
            }

            Ok(RewardReconciliationResult {
                candidate_id: candidate.id,
                wallet_credit_created,
                notification_created,
                external_transaction_link_repaired,
                internal_transaction_link_repaired,
                final_status: candidate.status,
            })
        })
    })
    .await?;

    log::info!(
        "event=reward_reconciled candidate_id={} wallet_credit_created={} notification_created={} external_transaction_link_repaired={} internal_transaction_link_repaired={} final_status={}",
        result.candidate_id,
        result.wallet_credit_created,
        result.notification_created,
        result.external_transaction_link_repaired,
        result.internal_transaction_link_repaired,
        result.final_status
    );

    Ok(result)
}
