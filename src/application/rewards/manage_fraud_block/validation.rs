use crate::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, RewardFraudBlockDraft, RewardFraudBlockError,
    RewardFraudBlockListFilter,
};
use crate::domain::rewards::fraud_block::RewardFraudBlockScope;

pub(super) fn validated_draft(
    actor_user_id: i32,
    command: CreateRewardFraudBlockCommand,
) -> Result<RewardFraudBlockDraft, RewardFraudBlockError> {
    let scope_type = normalized_scope(&command.scope_type)?;
    let reason = command.reason.trim().to_string();
    if reason.is_empty() {
        return Err(RewardFraudBlockError::InvalidInput(
            "block reason is required".to_string(),
        ));
    }

    ensure_exactly_one_target(&command)?;
    ensure_target_matches_scope(scope_type, &command)?;

    Ok(RewardFraudBlockDraft {
        created_by_user_id: actor_user_id,
        scope_type,
        teacher_user_id: command.teacher_user_id,
        organization_id: command.organization_id,
        course_id: command.course_id,
        reward_policy_id: command.reward_policy_id,
        reason,
        evidence_reference: trimmed_optional(command.evidence_reference),
        expires_at: command.expires_at,
    })
}

pub(super) fn validated_filter(
    query: crate::application::rewards::manage_fraud_block::ListRewardFraudBlocksQuery,
) -> Result<RewardFraudBlockListFilter, RewardFraudBlockError> {
    Ok(RewardFraudBlockListFilter {
        scope_type: query
            .scope_type
            .map(|scope| normalized_scope(&scope))
            .transpose()?,
        teacher_user_id: query.teacher_user_id,
        organization_id: query.organization_id,
        course_id: query.course_id,
        reward_policy_id: query.reward_policy_id,
        active: query.active,
        limit: query.limit,
        offset: query.offset,
    })
}

fn normalized_scope(scope_type: &str) -> Result<RewardFraudBlockScope, RewardFraudBlockError> {
    RewardFraudBlockScope::normalize(scope_type).map_err(|_| {
        RewardFraudBlockError::InvalidInput("unsupported fraud block scope type".to_string())
    })
}

fn ensure_exactly_one_target(
    command: &CreateRewardFraudBlockCommand,
) -> Result<(), RewardFraudBlockError> {
    let target_count = [
        command.teacher_user_id.is_some(),
        command.organization_id.is_some(),
        command.course_id.is_some(),
        command.reward_policy_id.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();

    if target_count == 1 {
        Ok(())
    } else {
        Err(RewardFraudBlockError::InvalidInput(
            "exactly one fraud block target is required".to_string(),
        ))
    }
}

fn ensure_target_matches_scope(
    scope_type: RewardFraudBlockScope,
    command: &CreateRewardFraudBlockCommand,
) -> Result<(), RewardFraudBlockError> {
    if scope_type.matches_target(
        command.teacher_user_id.is_some(),
        command.organization_id.is_some(),
        command.course_id.is_some(),
        command.reward_policy_id.is_some(),
    ) {
        Ok(())
    } else {
        Err(RewardFraudBlockError::InvalidInput(
            "fraud block target does not match scope type".to_string(),
        ))
    }
}

fn trimmed_optional(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests;
