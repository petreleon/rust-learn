use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::domain::access_control::role_assignment_audit::PlatformRoleAssignmentAuditEventType;
use crate::infra::postgres::models::platform_role_assignment_audit_event::NewPlatformRoleAssignmentAuditEvent;
use crate::infra::postgres::schema::platform_role_assignment_audit_events;

pub(super) async fn record_role_assignment_audit(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    target_user_id: i32,
    platform_role_id: i32,
    role_name: &str,
) -> diesel::QueryResult<usize> {
    diesel::insert_into(platform_role_assignment_audit_events::table)
        .values(NewPlatformRoleAssignmentAuditEvent {
            actor_user_id: Some(actor_user_id),
            event_type: PlatformRoleAssignmentAuditEventType::RoleAssigned
                .as_str()
                .to_string(),
            platform_role_id: Some(platform_role_id),
            role_name: role_name.to_string(),
            target_user_id,
        })
        .execute(conn)
        .await
}
