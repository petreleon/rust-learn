use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, organizations};
use crate::models::reward_candidate::{
    REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT, REWARD_EVENT_ASSESSMENT_COMPLETION,
    REWARD_EVENT_COURSE_COMPLETION, REWARD_EVENT_MANUAL_COMPLETION,
};
use crate::models::reward_policy::{
    NewRewardPolicy, RewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN,
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION,
    REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_policy_repository::{self, RewardPolicyFilter};
use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;

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

#[derive(Debug, PartialEq, Eq)]
pub enum RewardPolicyError {
    PermissionDenied(String),
    InvalidInput(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for RewardPolicyError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardPolicyError::NotFound,
            other => RewardPolicyError::Database(other.to_string()),
        }
    }
}

pub async fn create_reward_policy(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: CreateRewardPolicyRequest,
) -> Result<RewardPolicy, RewardPolicyError> {
    ensure_platform_permission(conn, actor_user_id, Permissions::SET_REWARD_POLICY).await?;

    let scope_type = normalize_scope_type(&request.scope_type)?;
    validate_scope_references(
        conn,
        &scope_type,
        request.organization_id,
        request.course_id,
    )
    .await?;
    let event_type = normalize_event_type(&request.event_type)?;
    let payment_strategy = normalize_payment_strategy(&request.payment_strategy)?;
    let multiplier = request.multiplier.unwrap_or_else(|| BigDecimal::from(1));
    let cooldown_seconds = request.cooldown_seconds.unwrap_or(0);
    let active = request.active.unwrap_or(true);
    validate_amounts(
        &request.token_amount,
        &multiplier,
        request.max_payout.as_ref(),
        cooldown_seconds,
    )?;

    conn.transaction::<_, RewardPolicyError, _>(|conn| {
        Box::pin(async move {
            let version = reward_policy_repository::next_policy_version(
                conn,
                &scope_type,
                request.organization_id,
                request.course_id,
                &event_type,
            )
            .await?;

            if active {
                reward_policy_repository::deactivate_active_policies(
                    conn,
                    &scope_type,
                    request.organization_id,
                    request.course_id,
                    &event_type,
                    Utc::now(),
                )
                .await?;
            }

            reward_policy_repository::create_policy(
                conn,
                NewRewardPolicy {
                    scope_type,
                    organization_id: request.organization_id,
                    course_id: request.course_id,
                    event_type,
                    version,
                    token_amount: request.token_amount,
                    multiplier,
                    max_payout: request.max_payout,
                    cooldown_seconds,
                    payment_strategy,
                    active,
                    created_by_user_id: Some(actor_user_id),
                },
            )
            .await
            .map_err(RewardPolicyError::from)
        })
    })
    .await
}
