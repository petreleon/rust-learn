use crate::db::schema::{courses_organizations, reward_policies};
use crate::models::reward_candidate::{RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED};
use crate::models::reward_policy::{
    RewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::repositories::persistent_state_repository::get_persistent_state;
use crate::repositories::reward_candidate_repository;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub const REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER: &str = "presigner_transfer";
pub const REWARD_PAYOUT_METHOD_TREASURY_TRANSFER: &str = "treasury_transfer";
pub const REWARD_PAYOUT_METHOD_MINT: &str = "mint";
pub const REWARD_PAYOUT_METHOD_OFF_CHAIN: &str = "off_chain";

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutPlan {
    pub candidate_id: i64,
    pub policy_id: i64,
    pub amount: BigDecimal,
    pub payment_strategy: String,
    pub payout_method: String,
    pub requires_token_confirmation: bool,
}

#[derive(Debug, PartialEq)]
pub enum RewardExecutionError {
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    Database(String),
}

impl From<diesel::result::Error> for RewardExecutionError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardExecutionError::NoActivePolicy,
            other => RewardExecutionError::Database(other.to_string()),
        }
    }
}

pub async fn plan_reward_payout(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    let candidate = reward_candidate_repository::find_candidate(conn, candidate_id).await?;
    ensure_candidate_ready_for_payout(&candidate)?;

    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardExecutionError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardExecutionError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }

    let policy = resolve_active_reward_policy(conn, candidate.course_id, &candidate.event_type)
        .await?
        .ok_or(RewardExecutionError::NoActivePolicy)?;
    let payout_method = select_payout_method(conn, &policy).await?;
    let requires_token_confirmation = payout_method != REWARD_PAYOUT_METHOD_OFF_CHAIN;

    Ok(RewardPayoutPlan {
        candidate_id: candidate.id,
        policy_id: policy.id,
        amount,
        payment_strategy: policy.payment_strategy,
        payout_method,
        requires_token_confirmation,
    })
}

fn ensure_candidate_ready_for_payout(
    candidate: &RewardCandidate,
) -> Result<(), RewardExecutionError> {
    if candidate.status == REWARD_STATUS_AMOUNT_APPROVED {
        Ok(())
    } else {
        Err(RewardExecutionError::InvalidStatus(
            "reward candidate must be amount approved before payout planning".to_string(),
        ))
    }
}

async fn select_payout_method(
    conn: &mut AsyncPgConnection,
    policy: &RewardPolicy,
) -> Result<String, RewardExecutionError> {
    match policy.payment_strategy.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER => {
            if has_presigner_contract(conn).await? {
                Ok(REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER.to_string())
            } else {
                Ok(REWARD_PAYOUT_METHOD_TREASURY_TRANSFER.to_string())
            }
        }
        REWARD_PAYMENT_MINT => Ok(REWARD_PAYOUT_METHOD_MINT.to_string()),
        REWARD_PAYMENT_OFF_CHAIN => Ok(REWARD_PAYOUT_METHOD_OFF_CHAIN.to_string()),
        _ => Err(RewardExecutionError::InvalidInput(
            "unsupported reward payment strategy".to_string(),
        )),
    }
}

async fn has_presigner_contract(
    conn: &mut AsyncPgConnection,
) -> Result<bool, RewardExecutionError> {
    let address = get_persistent_state(conn, "learn_token_presigner_address").await?;
    Ok(address
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false))
}

async fn resolve_active_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> QueryResult<Option<RewardPolicy>> {
    if let Some(policy) = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()?
    {
        return Ok(Some(policy));
    }

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        if let Some(policy) = reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
            .filter(reward_policies::organization_id.eq_any(organization_ids))
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true))
            .order((
                reward_policies::version.desc(),
                reward_policies::created_at.desc(),
            ))
            .first::<RewardPolicy>(conn)
            .await
            .optional()?
        {
            return Ok(Some(policy));
        }
    }

    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
        .filter(reward_policies::organization_id.is_null())
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()
}
