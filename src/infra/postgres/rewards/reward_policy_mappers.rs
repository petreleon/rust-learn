use crate::application::rewards::manage_reward_policy::{
    RewardPolicyDraft, RewardPolicyError, RewardPolicyListFilter, RewardPolicyOutput,
};
use crate::infra::postgres::rewards::reward_policy_records::RewardPolicyFilter;
use crate::infra::postgres::rewards::reward_vocabulary::{
    parse_payment_strategy, parse_policy_scope, parse_reward_event_type,
};
use crate::models::reward_policy::{NewRewardPolicy, RewardPolicy};

pub(super) fn new_reward_policy(draft: RewardPolicyDraft, version: i32) -> NewRewardPolicy {
    NewRewardPolicy {
        scope_type: draft.scope_type.as_str().to_string(),
        organization_id: draft.organization_id,
        course_id: draft.course_id,
        event_type: draft.event_type.as_str().to_string(),
        version,
        token_amount: draft.token_amount,
        multiplier: draft.multiplier,
        max_payout: draft.max_payout,
        cooldown_seconds: draft.cooldown_seconds,
        payment_strategy: draft.payment_strategy.as_str().to_string(),
        active: draft.active,
        created_by_user_id: Some(draft.actor_user_id),
    }
}

impl From<RewardPolicyListFilter> for RewardPolicyFilter {
    fn from(filter: RewardPolicyListFilter) -> Self {
        Self {
            scope_type: filter
                .scope_type
                .map(|scope_type| scope_type.as_str().to_string()),
            organization_id: filter.organization_id,
            course_id: filter.course_id,
            event_type: filter
                .event_type
                .map(|event_type| event_type.as_str().to_string()),
            active: filter.active,
            limit: filter.limit,
            offset: filter.offset,
        }
    }
}

impl TryFrom<RewardPolicy> for RewardPolicyOutput {
    type Error = RewardPolicyError;

    fn try_from(policy: RewardPolicy) -> Result<Self, Self::Error> {
        Ok(Self {
            id: policy.id,
            scope_type: parse_policy_scope(&policy.scope_type, RewardPolicyError::Database)?,
            organization_id: policy.organization_id,
            course_id: policy.course_id,
            event_type: parse_reward_event_type(&policy.event_type, RewardPolicyError::Database)?,
            version: policy.version,
            token_amount: policy.token_amount,
            multiplier: policy.multiplier,
            max_payout: policy.max_payout,
            cooldown_seconds: policy.cooldown_seconds,
            payment_strategy: parse_payment_strategy(
                &policy.payment_strategy,
                RewardPolicyError::Database,
            )?,
            active: policy.active,
            created_by_user_id: policy.created_by_user_id,
            created_at: policy.created_at,
            updated_at: policy.updated_at,
        })
    }
}

pub(super) fn map_reward_policy_error(error: diesel::result::Error) -> RewardPolicyError {
    match error {
        diesel::result::Error::NotFound => RewardPolicyError::NotFound,
        other => RewardPolicyError::Database(other.to_string()),
    }
}
