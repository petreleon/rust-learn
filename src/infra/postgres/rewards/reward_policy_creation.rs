use chrono::Utc;
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::application::rewards::manage_reward_policy::{
    RewardPolicyDraft, RewardPolicyError, RewardPolicyOutput,
};
use crate::domain::rewards::policy::RewardPolicyAuditEventType;
use crate::infra::postgres::rewards::reward_policy_activation::{
    active_scope_policies, deactivate_active_policies,
};
use crate::infra::postgres::rewards::reward_policy_audit_insert::insert_policy_audit;
use crate::infra::postgres::rewards::reward_policy_mappers::{
    map_reward_policy_error, new_reward_policy,
};
use crate::infra::postgres::rewards::reward_policy_records::{create_policy, next_policy_version};

pub(super) async fn create_versioned_policy(
    conn: &mut AsyncPgConnection,
    draft: RewardPolicyDraft,
) -> Result<RewardPolicyOutput, RewardPolicyError> {
    let policy = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let scope_type = draft.scope_type.as_str();
                let event_type = draft.event_type.as_str();
                let actor_user_id = draft.actor_user_id;
                let version = next_policy_version(
                    conn,
                    scope_type,
                    draft.organization_id,
                    draft.course_id,
                    event_type,
                )
                .await?;
                let replaced = active_policies_if_needed(conn, &draft).await?;

                if draft.active {
                    deactivate_active_policies(
                        conn,
                        scope_type,
                        draft.organization_id,
                        draft.course_id,
                        event_type,
                        Utc::now(),
                    )
                    .await?;
                }

                for policy in replaced {
                    insert_policy_audit(
                        conn,
                        policy.id,
                        actor_user_id,
                        RewardPolicyAuditEventType::Deactivated,
                        Some(true),
                        false,
                    )
                    .await?;
                }

                let created = create_policy(conn, new_reward_policy(draft, version)).await?;
                insert_policy_audit(
                    conn,
                    created.id,
                    actor_user_id,
                    RewardPolicyAuditEventType::Created,
                    None,
                    created.active,
                )
                .await?;
                Ok(created)
            })
        })
        .await
        .map_err(map_reward_policy_error)?;
    RewardPolicyOutput::try_from(policy)
}

async fn active_policies_if_needed(
    conn: &mut AsyncPgConnection,
    draft: &RewardPolicyDraft,
) -> diesel::QueryResult<Vec<crate::infra::postgres::models::reward_policy::RewardPolicy>> {
    if !draft.active {
        return Ok(Vec::new());
    }
    active_scope_policies(
        conn,
        draft.scope_type.as_str(),
        draft.organization_id,
        draft.course_id,
        draft.event_type.as_str(),
    )
    .await
}
