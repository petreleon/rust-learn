use chrono::Utc;
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::application::rewards::manage_reward_policy::{
    RewardPolicyError, RewardPolicyOutput, UpdateRewardPolicyActivationCommand,
};
use crate::domain::rewards::policy::RewardPolicyAuditEventType;
use crate::infra::postgres::models::reward_policy::RewardPolicy;
use crate::infra::postgres::rewards::reward_policy_activation::{
    active_scope_policies, deactivate_active_policies, set_policy_active,
};
use crate::infra::postgres::rewards::reward_policy_audit_insert::insert_policy_audit;
use crate::infra::postgres::rewards::reward_policy_mappers::map_reward_policy_error;
use crate::infra::postgres::rewards::reward_policy_records::find_policy;

pub(super) async fn update_policy_activation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    command: UpdateRewardPolicyActivationCommand,
) -> Result<RewardPolicyOutput, RewardPolicyError> {
    let policy = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let policy = find_policy(conn, command.policy_id).await?;
                if policy.active == command.active {
                    return Ok(policy);
                }
                if command.active {
                    deactivate_sibling_policies(conn, &policy, actor_user_id).await?;
                }
                let updated =
                    set_policy_active(conn, policy.id, command.active, Utc::now()).await?;
                audit_target_policy(conn, &policy, &updated, actor_user_id).await?;
                Ok(updated)
            })
        })
        .await
        .map_err(map_reward_policy_error)?;
    RewardPolicyOutput::try_from(policy)
}

async fn audit_target_policy(
    conn: &mut AsyncPgConnection,
    previous: &RewardPolicy,
    updated: &RewardPolicy,
    actor_user_id: i32,
) -> diesel::QueryResult<()> {
    let event_type = if updated.active {
        RewardPolicyAuditEventType::Activated
    } else {
        RewardPolicyAuditEventType::Deactivated
    };
    insert_policy_audit(
        conn,
        updated.id,
        actor_user_id,
        event_type,
        Some(previous.active),
        updated.active,
    )
    .await
}

async fn deactivate_sibling_policies(
    conn: &mut AsyncPgConnection,
    policy: &RewardPolicy,
    actor_user_id: i32,
) -> diesel::QueryResult<()> {
    let siblings = active_scope_policies(
        conn,
        &policy.scope_type,
        policy.organization_id,
        policy.course_id,
        &policy.event_type,
    )
    .await?;
    deactivate_active_policies(
        conn,
        &policy.scope_type,
        policy.organization_id,
        policy.course_id,
        &policy.event_type,
        Utc::now(),
    )
    .await?;
    for sibling in siblings {
        insert_policy_audit(
            conn,
            sibling.id,
            actor_user_id,
            RewardPolicyAuditEventType::Deactivated,
            Some(true),
            false,
        )
        .await?;
    }
    Ok(())
}
