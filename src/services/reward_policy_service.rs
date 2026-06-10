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

pub async fn list_reward_policies(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListRewardPoliciesRequest,
) -> Result<Vec<RewardPolicy>, RewardPolicyError> {
    ensure_platform_permission(conn, actor_user_id, Permissions::SET_REWARD_POLICY).await?;

    let scope_type = match request.scope_type {
        Some(scope_type) => Some(normalize_scope_type(&scope_type)?),
        None => None,
    };
    let event_type = match request.event_type {
        Some(event_type) => Some(normalize_event_type(&event_type)?),
        None => None,
    };

    reward_policy_repository::list_policies(
        conn,
        RewardPolicyFilter {
            scope_type,
            organization_id: request.organization_id,
            course_id: request.course_id,
            event_type,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(RewardPolicyError::from)
}

async fn ensure_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: Permissions,
) -> Result<(), RewardPolicyError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, user_id, &permission_name).await? {
        Ok(())
    } else {
        Err(RewardPolicyError::PermissionDenied(permission_name))
    }
}

async fn validate_scope_references(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(), RewardPolicyError> {
    match scope_type {
        REWARD_POLICY_SCOPE_PLATFORM => {
            if organization_id.is_some() || course_id.is_some() {
                return Err(RewardPolicyError::InvalidInput(
                    "platform reward policy cannot include organization or course scope"
                        .to_string(),
                ));
            }
        }
        REWARD_POLICY_SCOPE_ORGANIZATION => {
            let organization_id = organization_id.ok_or_else(|| {
                RewardPolicyError::InvalidInput(
                    "organization reward policy requires organization_id".to_string(),
                )
            })?;
            if course_id.is_some() {
                return Err(RewardPolicyError::InvalidInput(
                    "organization reward policy cannot include course_id".to_string(),
                ));
            }
            organizations::table
                .find(organization_id)
                .select(organizations::id)
                .first::<i32>(conn)
                .await?;
        }
        REWARD_POLICY_SCOPE_COURSE => {
            let course_id = course_id.ok_or_else(|| {
                RewardPolicyError::InvalidInput(
                    "course reward policy requires course_id".to_string(),
                )
            })?;
            courses::table
                .find(course_id)
                .select(courses::id)
                .first::<i32>(conn)
                .await?;
            if let Some(organization_id) = organization_id {
                organizations::table
                    .find(organization_id)
                    .select(organizations::id)
                    .first::<i32>(conn)
                    .await?;
            }
        }
        _ => unreachable!("scope type was normalized before validation"),
    }

    Ok(())
}

fn validate_amounts(
    token_amount: &BigDecimal,
    multiplier: &BigDecimal,
    max_payout: Option<&BigDecimal>,
    cooldown_seconds: i64,
) -> Result<(), RewardPolicyError> {
    if token_amount < &BigDecimal::from(0) {
        return Err(RewardPolicyError::InvalidInput(
            "token amount cannot be negative".to_string(),
        ));
    }
    if multiplier <= &BigDecimal::from(0) {
        return Err(RewardPolicyError::InvalidInput(
            "multiplier must be greater than zero".to_string(),
        ));
    }
    if let Some(max_payout) = max_payout {
        if max_payout < &BigDecimal::from(0) {
            return Err(RewardPolicyError::InvalidInput(
                "max payout cannot be negative".to_string(),
            ));
        }
    }
    if cooldown_seconds < 0 {
        return Err(RewardPolicyError::InvalidInput(
            "cooldown seconds cannot be negative".to_string(),
        ));
    }

    Ok(())
}

fn normalize_scope_type(scope_type: &str) -> Result<String, RewardPolicyError> {
    let normalized = scope_type.trim().to_ascii_lowercase();
    match normalized.as_str() {
        REWARD_POLICY_SCOPE_PLATFORM
        | REWARD_POLICY_SCOPE_ORGANIZATION
        | REWARD_POLICY_SCOPE_COURSE => Ok(normalized),
        _ => Err(RewardPolicyError::InvalidInput(
            "unsupported reward policy scope type".to_string(),
        )),
    }
}

fn normalize_event_type(event_type: &str) -> Result<String, RewardPolicyError> {
    let normalized = event_type
        .trim()
        .to_ascii_lowercase()
        .replace(['-', ' '], "_");
    match normalized.as_str() {
        REWARD_EVENT_ASSESSMENT_COMPLETION
        | REWARD_EVENT_COURSE_COMPLETION
        | REWARD_EVENT_MANUAL_COMPLETION
        | REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT => Ok(normalized),
        _ => Err(RewardPolicyError::InvalidInput(
            "unsupported reward policy event type".to_string(),
        )),
    }
}

fn normalize_payment_strategy(payment_strategy: &str) -> Result<String, RewardPolicyError> {
    let normalized = payment_strategy.trim().to_ascii_lowercase();
    match normalized.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER | REWARD_PAYMENT_MINT | REWARD_PAYMENT_OFF_CHAIN => {
            Ok(normalized)
        }
        _ => Err(RewardPolicyError::InvalidInput(
            "unsupported reward policy payment strategy".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    // ── validate_amounts ──

    fn bd(value: i64) -> BigDecimal {
        BigDecimal::from(value)
    }

    #[test]
    fn valid_amounts_pass() {
        validate_amounts(&bd(100), &bd(2), Some(&bd(500)), 3600).unwrap();
    }

    #[test]
    fn zero_token_amount_is_valid() {
        validate_amounts(&bd(0), &bd(1), None, 0).unwrap();
    }

    #[test]
    fn negative_token_amount_fails() {
        assert!(validate_amounts(&bd(-1), &bd(1), None, 0).is_err());
    }

    #[test]
    fn zero_multiplier_fails() {
        assert!(validate_amounts(&bd(100), &bd(0), None, 0).is_err());
    }

    #[test]
    fn negative_multiplier_fails() {
        assert!(validate_amounts(&bd(100), &bd(-1), None, 0).is_err());
    }

    #[test]
    fn negative_max_payout_fails() {
        assert!(validate_amounts(&bd(100), &bd(1), Some(&bd(-1)), 0).is_err());
    }

    #[test]
    fn none_max_payout_is_valid() {
        validate_amounts(&bd(100), &bd(1), None, 0).unwrap();
    }

    #[test]
    fn negative_cooldown_fails() {
        assert!(validate_amounts(&bd(100), &bd(1), None, -1).is_err());
    }

    // ── normalize_scope_type ──

    #[test]
    fn normalizes_valid_policy_scopes() {
        assert_eq!(normalize_scope_type(REWARD_POLICY_SCOPE_PLATFORM).unwrap(), "platform");
        assert_eq!(normalize_scope_type(REWARD_POLICY_SCOPE_ORGANIZATION).unwrap(), "organization");
        assert_eq!(normalize_scope_type(REWARD_POLICY_SCOPE_COURSE).unwrap(), "course");
        assert_eq!(normalize_scope_type("  COURSE  ").unwrap(), "course");
    }

    #[test]
    fn rejects_invalid_policy_scope() {
        assert!(normalize_scope_type("").is_err());
        assert!(normalize_scope_type("unknown").is_err());
    }

    // ── normalize_event_type ──

    #[test]
    fn normalizes_valid_event_types() {
        assert_eq!(normalize_event_type(REWARD_EVENT_COURSE_COMPLETION).unwrap(), "course_completion");
        assert_eq!(normalize_event_type(REWARD_EVENT_ASSESSMENT_COMPLETION).unwrap(), "assessment_completion");
        assert_eq!(normalize_event_type(REWARD_EVENT_MANUAL_COMPLETION).unwrap(), "manual_completion");
        assert_eq!(normalize_event_type(REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT).unwrap(), "administrative_adjustment");
    }

    #[test]
    fn normalizes_event_type_whitespace_and_dashes() {
        assert_eq!(normalize_event_type("  course-completion  ").unwrap(), "course_completion");
        assert_eq!(normalize_event_type("assessment completion").unwrap(), "assessment_completion");
    }

    #[test]
    fn rejects_invalid_event_type() {
        assert!(normalize_event_type("").is_err());
        assert!(normalize_event_type("unknown").is_err());
    }

    // ── normalize_payment_strategy ──

    #[test]
    fn normalizes_valid_strategies() {
        assert_eq!(normalize_payment_strategy(REWARD_PAYMENT_TREASURY_TRANSFER).unwrap(), "treasury_transfer");
        assert_eq!(normalize_payment_strategy(REWARD_PAYMENT_MINT).unwrap(), "mint");
        assert_eq!(normalize_payment_strategy(REWARD_PAYMENT_OFF_CHAIN).unwrap(), "off_chain");
        assert_eq!(normalize_payment_strategy("  MINT  ").unwrap(), "mint");
    }

    #[test]
    fn rejects_invalid_strategy() {
        assert!(normalize_payment_strategy("").is_err());
        assert!(normalize_payment_strategy("direct_transfer").is_err());
    }
}
