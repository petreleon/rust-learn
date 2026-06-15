use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyOutput,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRewardPolicyRequest {
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: String,
    pub token_amount: BigDecimal,
    pub multiplier: Option<BigDecimal>,
    pub max_payout: Option<BigDecimal>,
    pub cooldown_seconds: Option<i64>,
    pub payment_strategy: String,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListRewardPoliciesRequest {
    pub scope_type: Option<String>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: Option<String>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardPolicyResponse {
    pub id: i64,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: String,
    pub version: i32,
    pub token_amount: BigDecimal,
    pub multiplier: BigDecimal,
    pub max_payout: Option<BigDecimal>,
    pub cooldown_seconds: i64,
    pub payment_strategy: String,
    pub active: bool,
    pub created_by_user_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CreateRewardPolicyRequest> for CreateRewardPolicyCommand {
    fn from(request: CreateRewardPolicyRequest) -> Self {
        Self {
            scope_type: request.scope_type,
            organization_id: request.organization_id,
            course_id: request.course_id,
            event_type: request.event_type,
            token_amount: request.token_amount,
            multiplier: request.multiplier,
            max_payout: request.max_payout,
            cooldown_seconds: request.cooldown_seconds,
            payment_strategy: request.payment_strategy,
            active: request.active,
        }
    }
}

impl From<ListRewardPoliciesRequest> for ListRewardPoliciesQuery {
    fn from(request: ListRewardPoliciesRequest) -> Self {
        Self {
            scope_type: request.scope_type,
            organization_id: request.organization_id,
            course_id: request.course_id,
            event_type: request.event_type,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        }
    }
}

impl From<RewardPolicyOutput> for RewardPolicyResponse {
    fn from(policy: RewardPolicyOutput) -> Self {
        Self {
            id: policy.id,
            scope_type: policy.scope_type.as_str().to_string(),
            organization_id: policy.organization_id,
            course_id: policy.course_id,
            event_type: policy.event_type.as_str().to_string(),
            version: policy.version,
            token_amount: policy.token_amount,
            multiplier: policy.multiplier,
            max_payout: policy.max_payout,
            cooldown_seconds: policy.cooldown_seconds,
            payment_strategy: policy.payment_strategy.as_str().to_string(),
            active: policy.active,
            created_by_user_id: policy.created_by_user_id,
            created_at: policy.created_at,
            updated_at: policy.updated_at,
        }
    }
}
