use crate::application::rewards::manage_fraud_block::{
    RewardFraudBlockDraft, RewardFraudBlockError, RewardFraudBlockListFilter,
    RewardFraudBlockOutput,
};
use crate::infra::postgres::models::reward_fraud_block::{NewRewardFraudBlock, RewardFraudBlock};
use crate::infra::postgres::rewards::reward_fraud_block_records::RewardFraudBlockFilter;
use crate::infra::postgres::rewards::reward_vocabulary::parse_fraud_block_scope;

pub(super) fn new_reward_fraud_block(draft: RewardFraudBlockDraft) -> NewRewardFraudBlock {
    NewRewardFraudBlock {
        scope_type: draft.scope_type.as_str().to_string(),
        teacher_user_id: draft.teacher_user_id,
        organization_id: draft.organization_id,
        course_id: draft.course_id,
        reward_policy_id: draft.reward_policy_id,
        reason: draft.reason,
        evidence_reference: draft.evidence_reference,
        created_by_user_id: draft.created_by_user_id,
        expires_at: draft.expires_at,
    }
}

impl From<RewardFraudBlockListFilter> for RewardFraudBlockFilter {
    fn from(filter: RewardFraudBlockListFilter) -> Self {
        Self {
            scope_type: filter
                .scope_type
                .map(|scope_type| scope_type.as_str().to_string()),
            teacher_user_id: filter.teacher_user_id,
            organization_id: filter.organization_id,
            course_id: filter.course_id,
            reward_policy_id: filter.reward_policy_id,
            active: filter.active,
            limit: filter.limit,
            offset: filter.offset,
        }
    }
}

impl TryFrom<RewardFraudBlock> for RewardFraudBlockOutput {
    type Error = RewardFraudBlockError;

    fn try_from(block: RewardFraudBlock) -> Result<Self, Self::Error> {
        Ok(Self {
            id: block.id,
            scope_type: parse_fraud_block_scope(
                &block.scope_type,
                RewardFraudBlockError::Database,
            )?,
            teacher_user_id: block.teacher_user_id,
            organization_id: block.organization_id,
            course_id: block.course_id,
            reward_policy_id: block.reward_policy_id,
            reason: block.reason,
            evidence_reference: block.evidence_reference,
            created_by_user_id: block.created_by_user_id,
            expires_at: block.expires_at,
            revoked_by_user_id: block.revoked_by_user_id,
            revoked_at: block.revoked_at,
            created_at: block.created_at,
            updated_at: block.updated_at,
        })
    }
}

pub(super) fn map_reward_fraud_block_error(error: diesel::result::Error) -> RewardFraudBlockError {
    match error {
        diesel::result::Error::NotFound => RewardFraudBlockError::NotFound,
        other => RewardFraudBlockError::Database(other.to_string()),
    }
}
