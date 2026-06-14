use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::access_control::permission_checks::{
    has_organization_permission as access_control_has_organization_permission,
    has_platform_permission as access_control_has_platform_permission,
};

pub(super) async fn has_platform_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: Permissions,
) -> QueryResult<bool> {
    has_platform_permission_name(conn, actor_user_id, &permission.to_string()).await
}

pub(super) async fn has_platform_permission_name(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    access_control_has_platform_permission(conn, actor_user_id, permission).await
}

pub(super) async fn has_organization_permission_name(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    access_control_has_organization_permission(conn, actor_user_id, organization_id, permission)
        .await
}
