use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyError, RewardPolicyOutput,
    UpdateRewardPolicyActivationCommand,
};
use crate::domain::rewards::policy::{
    RewardPaymentStrategy, RewardPolicyEventType, RewardPolicyScope,
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

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRewardPolicyActivationRequest {
    pub active: bool,
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

impl CreateRewardPolicyRequest {
    pub(in crate::http::rewards) fn into_command(
        self,
    ) -> Result<CreateRewardPolicyCommand, RewardPolicyError> {
        Ok(CreateRewardPolicyCommand {
            scope_type: reward_policy_scope(&self.scope_type)?,
            organization_id: self.organization_id,
            course_id: self.course_id,
            event_type: reward_policy_event_type(&self.event_type)?,
            token_amount: self.token_amount,
            multiplier: self.multiplier,
            max_payout: self.max_payout,
            cooldown_seconds: self.cooldown_seconds,
            payment_strategy: reward_payment_strategy(&self.payment_strategy)?,
            active: self.active,
        })
    }
}

impl UpdateRewardPolicyActivationRequest {
    pub(in crate::http::rewards) fn into_command(
        self,
        policy_id: i64,
    ) -> UpdateRewardPolicyActivationCommand {
        UpdateRewardPolicyActivationCommand {
            policy_id,
            active: self.active,
        }
    }
}

impl ListRewardPoliciesRequest {
    pub(in crate::http::rewards) fn into_query(
        self,
    ) -> Result<ListRewardPoliciesQuery, RewardPolicyError> {
        Ok(ListRewardPoliciesQuery {
            scope_type: request_optional_scope(self.scope_type)?,
            organization_id: self.organization_id,
            course_id: self.course_id,
            event_type: request_optional_event(self.event_type)?,
            active: self.active,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

fn reward_policy_scope(scope_type: &str) -> Result<RewardPolicyScope, RewardPolicyError> {
    RewardPolicyScope::normalize(scope_type).map_err(|_| {
        RewardPolicyError::InvalidInput("unsupported reward policy scope type".to_string())
    })
}

fn reward_policy_event_type(event_type: &str) -> Result<RewardPolicyEventType, RewardPolicyError> {
    RewardPolicyEventType::normalize(event_type).map_err(|_| {
        RewardPolicyError::InvalidInput("unsupported reward policy event type".to_string())
    })
}

fn reward_payment_strategy(
    payment_strategy: &str,
) -> Result<RewardPaymentStrategy, RewardPolicyError> {
    RewardPaymentStrategy::normalize(payment_strategy).map_err(|_| {
        RewardPolicyError::InvalidInput("unsupported reward policy payment strategy".to_string())
    })
}

fn request_optional_scope(
    scope_type: Option<String>,
) -> Result<Option<RewardPolicyScope>, RewardPolicyError> {
    scope_type.as_deref().map(reward_policy_scope).transpose()
}

fn request_optional_event(
    event_type: Option<String>,
) -> Result<Option<RewardPolicyEventType>, RewardPolicyError> {
    event_type
        .as_deref()
        .map(reward_policy_event_type)
        .transpose()
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
