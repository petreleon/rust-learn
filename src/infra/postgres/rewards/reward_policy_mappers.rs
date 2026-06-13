use crate::application::rewards::manage_reward_policy::{
    RewardPolicyDraft, RewardPolicyError, RewardPolicyListFilter, RewardPolicyOutput,
};
use crate::models::reward_policy::{NewRewardPolicy, RewardPolicy};
use crate::repositories::reward_policy_repository::RewardPolicyFilter;

pub(super) fn new_reward_policy(draft: RewardPolicyDraft, version: i32) -> NewRewardPolicy {
    NewRewardPolicy {
        scope_type: draft.scope_type,
        organization_id: draft.organization_id,
        course_id: draft.course_id,
        event_type: draft.event_type,
        version,
        token_amount: draft.token_amount,
        multiplier: draft.multiplier,
        max_payout: draft.max_payout,
        cooldown_seconds: draft.cooldown_seconds,
        payment_strategy: draft.payment_strategy,
        active: draft.active,
        created_by_user_id: Some(draft.actor_user_id),
    }
}

impl From<RewardPolicyListFilter> for RewardPolicyFilter {
    fn from(filter: RewardPolicyListFilter) -> Self {
        Self {
            scope_type: filter.scope_type,
            organization_id: filter.organization_id,
            course_id: filter.course_id,
            event_type: filter.event_type,
            active: filter.active,
            limit: filter.limit,
            offset: filter.offset,
        }
    }
}

impl From<RewardPolicy> for RewardPolicyOutput {
    fn from(policy: RewardPolicy) -> Self {
        Self {
            id: policy.id,
            scope_type: policy.scope_type,
            organization_id: policy.organization_id,
            course_id: policy.course_id,
            event_type: policy.event_type,
            version: policy.version,
            token_amount: policy.token_amount,
            multiplier: policy.multiplier,
            max_payout: policy.max_payout,
            cooldown_seconds: policy.cooldown_seconds,
            payment_strategy: policy.payment_strategy,
            active: policy.active,
            created_by_user_id: policy.created_by_user_id,
            created_at: policy.created_at,
            updated_at: policy.updated_at,
        }
    }
}

pub(super) fn map_reward_policy_error(error: diesel::result::Error) -> RewardPolicyError {
    match error {
        diesel::result::Error::NotFound => RewardPolicyError::NotFound,
        other => RewardPolicyError::Database(other.to_string()),
    }
}
