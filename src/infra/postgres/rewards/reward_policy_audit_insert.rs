use diesel_async::AsyncPgConnection;

use crate::domain::rewards::policy::RewardPolicyAuditEventType;
use crate::infra::postgres::rewards::reward_policy_audit_records::insert_policy_audit_event;
use crate::infra::postgres::rewards::reward_policy_mappers::new_reward_policy_audit_event;

pub(crate) async fn insert_policy_audit(
    conn: &mut AsyncPgConnection,
    policy_id: i64,
    actor_user_id: i32,
    event_type: RewardPolicyAuditEventType,
    previous_active: Option<bool>,
    new_active: bool,
) -> diesel::QueryResult<()> {
    insert_policy_audit_event(
        conn,
        new_reward_policy_audit_event(
            policy_id,
            actor_user_id,
            event_type,
            previous_active,
            new_active,
        ),
    )
    .await
    .map(|_| ())
}
