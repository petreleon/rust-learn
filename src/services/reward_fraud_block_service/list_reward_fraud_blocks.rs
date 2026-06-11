pub async fn list_reward_fraud_blocks(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListRewardFraudBlocksRequest,
) -> Result<ListRewardFraudBlocksResponse, RewardFraudBlockError> {
    ensure_reward_fraud_report_permission(conn, actor_user_id).await?;

    let scope_type = request
        .scope_type
        .as_deref()
        .map(normalize_scope_type)
        .transpose()?;

    let (blocks, total) = reward_fraud_block_repository::list_reward_fraud_blocks(
        conn,
        reward_fraud_block_repository::RewardFraudBlockFilter {
            scope_type,
            teacher_user_id: request.teacher_user_id,
            organization_id: request.organization_id,
            course_id: request.course_id,
            reward_policy_id: request.reward_policy_id,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(RewardFraudBlockError::from)?;

    let limit = request.limit.unwrap_or(100).clamp(1, 500);
    let offset = request.offset.unwrap_or(0).max(0);

    Ok(ListRewardFraudBlocksResponse {
        blocks,
        limit,
        offset,
        total,
    })
}

pub async fn reward_fraud_block_audit_history(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    block_id: i64,
) -> Result<Vec<RewardFraudBlockAuditEvent>, RewardFraudBlockError> {
    ensure_reward_fraud_report_permission(conn, actor_user_id).await?;
    let block = reward_fraud_block_repository::find_reward_fraud_block(conn, block_id).await?;

    let mut events = vec![RewardFraudBlockAuditEvent {
        fraud_block_id: block.id,
        event_type: "created".to_string(),
        actor_user_id: block.created_by_user_id,
        scope_type: block.scope_type.clone(),
        teacher_user_id: block.teacher_user_id,
        organization_id: block.organization_id,
        course_id: block.course_id,
        reward_policy_id: block.reward_policy_id,
        reason: block.reason.clone(),
        evidence_reference: block.evidence_reference.clone(),
        occurred_at: block.created_at,
    }];

    if let (Some(revoked_by_user_id), Some(revoked_at)) =
        (block.revoked_by_user_id, block.revoked_at)
    {
        events.push(RewardFraudBlockAuditEvent {
            fraud_block_id: block.id,
            event_type: "revoked".to_string(),
            actor_user_id: revoked_by_user_id,
            scope_type: block.scope_type,
            teacher_user_id: block.teacher_user_id,
            organization_id: block.organization_id,
            course_id: block.course_id,
            reward_policy_id: block.reward_policy_id,
            reason: block.reason,
            evidence_reference: block.evidence_reference,
            occurred_at: revoked_at,
        });
    }

    Ok(events)
}

async fn notify_reward_fraud_block_transition(
    conn: &mut AsyncPgConnection,
    block: &RewardFraudBlock,
    event_type: &str,
) -> Result<(), RewardFraudBlockError> {
    let recipients = reward_fraud_block_notification_recipients(conn, block).await?;
    if recipients.is_empty() {
        return Ok(());
    }

    let title = format!("reward_fraud_block:{}", event_type);
    let body = format!(
        "Reward fraud block #{} was {} for {} scope. Reason: {}",
        block.id, event_type, block.scope_type, block.reason
    );

    let notifications = recipients
        .into_iter()
        .map(|user_id| NewNotification {
            user_id: Some(user_id),
            title: title.as_str(),
            body: body.as_str(),
        })
        .collect::<Vec<_>>();
    create_notifications_bulk(conn, notifications.as_slice())
        .await
        .map_err(|e| RewardFraudBlockError::Database(e.to_string()))?;

    Ok(())
}

async fn reward_fraud_block_notification_recipients(
    conn: &mut AsyncPgConnection,
    block: &RewardFraudBlock,
) -> Result<HashSet<i32>, RewardFraudBlockError> {
    let mut recipients = HashSet::new();

    if let Some(teacher_user_id) = block.teacher_user_id {
        recipients.insert(teacher_user_id);
    }

    if let Some(organization_id) = block.organization_id {
        recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
    }

    if let Some(course_id) = block.course_id {
        for organization_id in course_organization_ids(conn, course_id).await? {
            recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
        }
    }

    if let Some(reward_policy_id) = block.reward_policy_id {
        recipients.extend(reward_policy_operator_user_ids(conn, reward_policy_id).await?);
    }

    recipients.extend(platform_reward_reviewer_user_ids(conn).await?);
    Ok(recipients)
}
