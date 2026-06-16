use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel_async::AsyncPgConnection;

use crate::application::learning::manage_course_completion_terms::CourseCompletionTermsError;
use crate::domain::rewards::policy::{
    RewardPaymentStrategy, RewardPolicyAuditEventType, RewardPolicyEventType, RewardPolicyScope,
};
use crate::infra::postgres::models::course_completion_terms::CourseCompletionTerms;
use crate::infra::postgres::models::reward_policy::NewRewardPolicy;
use crate::infra::postgres::rewards::reward_policy_activation::{
    active_scope_policies, deactivate_active_policies,
};
use crate::infra::postgres::rewards::reward_policy_audit_insert::insert_policy_audit;
use crate::infra::postgres::rewards::reward_policy_records::{create_policy, next_policy_version};

pub async fn create_completion_policy(
    conn: &mut AsyncPgConnection,
    terms: &CourseCompletionTerms,
    actor_user_id: i32,
) -> Result<i64, CourseCompletionTermsError> {
    let now = Utc::now();
    let scope_type = RewardPolicyScope::Course.as_str();
    let event_type = RewardPolicyEventType::CourseCompletion.as_str();
    let replaced = active_scope_policies(conn, scope_type, None, Some(terms.course_id), event_type)
        .await
        .map_err(map_reward_error)?;
    deactivate_active_policies(
        conn,
        scope_type,
        None,
        Some(terms.course_id),
        event_type,
        now,
    )
    .await
    .map_err(map_reward_error)?;

    for policy in replaced {
        insert_policy_audit(
            conn,
            policy.id,
            actor_user_id,
            RewardPolicyAuditEventType::Deactivated,
            Some(true),
            false,
        )
        .await
        .map_err(map_reward_error)?;
    }

    let version = next_policy_version(conn, scope_type, None, Some(terms.course_id), event_type)
        .await
        .map_err(map_reward_error)?;
    let policy = create_policy(conn, new_policy(terms, actor_user_id, version))
        .await
        .map_err(map_reward_error)?;
    insert_policy_audit(
        conn,
        policy.id,
        actor_user_id,
        RewardPolicyAuditEventType::Created,
        None,
        true,
    )
    .await
    .map_err(map_reward_error)?;
    Ok(policy.id)
}

fn new_policy(terms: &CourseCompletionTerms, actor_user_id: i32, version: i32) -> NewRewardPolicy {
    NewRewardPolicy {
        scope_type: RewardPolicyScope::Course.as_str().to_string(),
        organization_id: None,
        course_id: Some(terms.course_id),
        event_type: RewardPolicyEventType::CourseCompletion.as_str().to_string(),
        version,
        token_amount: terms.completion_reward_amount.clone(),
        multiplier: BigDecimal::from(1),
        max_payout: None,
        cooldown_seconds: 0,
        payment_strategy: RewardPaymentStrategy::TreasuryTransfer.as_str().to_string(),
        active: true,
        created_by_user_id: Some(actor_user_id),
    }
}

fn map_reward_error(error: diesel::result::Error) -> CourseCompletionTermsError {
    CourseCompletionTermsError::Database(error.to_string())
}
