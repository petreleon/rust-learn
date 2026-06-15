use crate::application::rewards::manage_fraud_block::validation::{
    validated_draft, validated_filter,
};
use crate::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, ListRewardFraudBlocksOutput, ListRewardFraudBlocksQuery,
    RewardFraudBlockAuditEventOutput, RewardFraudBlockError, RewardFraudBlockOutput,
};
use crate::application::rewards::ports::RewardFraudBlockStore;
use crate::domain::rewards::fraud_block::{RewardFraudBlockAuditEventType, RewardFraudBlockScope};

pub async fn create_reward_fraud_block(
    store: &mut impl RewardFraudBlockStore,
    actor_user_id: i32,
    command: CreateRewardFraudBlockCommand,
) -> Result<RewardFraudBlockOutput, RewardFraudBlockError> {
    let draft = validated_draft(actor_user_id, command)?;
    ensure_can_manage_scope(store, actor_user_id, draft.scope_type).await?;
    let block = store.create_fraud_block(draft).await?;
    store
        .notify_fraud_block_transition(&block, RewardFraudBlockAuditEventType::Created)
        .await?;
    Ok(block)
}

pub async fn list_reward_fraud_blocks(
    store: &mut impl RewardFraudBlockStore,
    actor_user_id: i32,
    query: ListRewardFraudBlocksQuery,
) -> Result<ListRewardFraudBlocksOutput, RewardFraudBlockError> {
    ensure_can_view(store, actor_user_id).await?;
    let limit = query.limit.unwrap_or(100).clamp(1, 500);
    let offset = query.offset.unwrap_or(0).max(0);
    let (blocks, total) = store.list_fraud_blocks(validated_filter(query)?).await?;

    Ok(ListRewardFraudBlocksOutput {
        blocks,
        limit,
        offset,
        total,
    })
}

pub async fn revoke_reward_fraud_block(
    store: &mut impl RewardFraudBlockStore,
    actor_user_id: i32,
    block_id: i64,
) -> Result<RewardFraudBlockOutput, RewardFraudBlockError> {
    let existing = store.find_fraud_block(block_id).await?;
    ensure_can_manage_scope(store, actor_user_id, existing.scope_type).await?;
    if existing.revoked_at.is_some() {
        return Ok(existing);
    }

    let block = store.revoke_fraud_block(block_id, actor_user_id).await?;
    store
        .notify_fraud_block_transition(&block, RewardFraudBlockAuditEventType::Revoked)
        .await?;
    Ok(block)
}

pub async fn reward_fraud_block_audit_history(
    store: &mut impl RewardFraudBlockStore,
    actor_user_id: i32,
    block_id: i64,
) -> Result<Vec<RewardFraudBlockAuditEventOutput>, RewardFraudBlockError> {
    ensure_can_view(store, actor_user_id).await?;
    let block = store.find_fraud_block(block_id).await?;
    Ok(audit_events(block))
}

async fn ensure_can_manage_scope(
    store: &mut impl RewardFraudBlockStore,
    actor_user_id: i32,
    scope_type: RewardFraudBlockScope,
) -> Result<(), RewardFraudBlockError> {
    if store
        .can_manage_fraud_block_scope(actor_user_id, scope_type)
        .await?
    {
        Ok(())
    } else {
        Err(RewardFraudBlockError::PermissionDenied(
            scope_type.as_str().to_string(),
        ))
    }
}

async fn ensure_can_view(
    store: &mut impl RewardFraudBlockStore,
    actor_user_id: i32,
) -> Result<(), RewardFraudBlockError> {
    if store.can_view_fraud_blocks(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardFraudBlockError::PermissionDenied(
            "VIEW_REWARD_AUDIT or MANAGE_REWARD_FRAUD_BLOCKS".to_string(),
        ))
    }
}

fn audit_events(block: RewardFraudBlockOutput) -> Vec<RewardFraudBlockAuditEventOutput> {
    let mut events = vec![RewardFraudBlockAuditEventOutput {
        fraud_block_id: block.id,
        event_type: RewardFraudBlockAuditEventType::Created,
        actor_user_id: block.created_by_user_id,
        scope_type: block.scope_type,
        teacher_user_id: block.teacher_user_id,
        organization_id: block.organization_id,
        course_id: block.course_id,
        reward_policy_id: block.reward_policy_id,
        reason: block.reason.clone(),
        evidence_reference: block.evidence_reference.clone(),
        occurred_at: block.created_at,
    }];

    if let (Some(actor_user_id), Some(occurred_at)) = (block.revoked_by_user_id, block.revoked_at) {
        events.push(RewardFraudBlockAuditEventOutput {
            fraud_block_id: block.id,
            event_type: RewardFraudBlockAuditEventType::Revoked,
            actor_user_id,
            scope_type: block.scope_type,
            teacher_user_id: block.teacher_user_id,
            organization_id: block.organization_id,
            course_id: block.course_id,
            reward_policy_id: block.reward_policy_id,
            reason: block.reason,
            evidence_reference: block.evidence_reference,
            occurred_at,
        });
    }

    events
}
