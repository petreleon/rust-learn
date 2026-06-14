use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{delegated_permissions, role_permission_platform, user_role_platform};

pub async fn user_has_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    if has_platform_role_permission(conn, user_id, permission).await? {
        return Ok(true);
    }

    let has_delegation = has_active_platform_delegation(conn, user_id, permission).await?;
    if has_delegation {
        log::info!(
            "event=delegated_permission_used scope=platform user_id={} permission={}",
            user_id,
            permission
        );
    }

    Ok(has_delegation)
}

async fn has_platform_role_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    select(exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(
                user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
            ))
            .filter(user_role_platform::user_id.eq(user_id))
            .filter(role_permission_platform::permission.eq(permission)),
    ))
    .get_result(conn)
    .await
}

async fn has_active_platform_delegation(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(user_id))
            .filter(delegated_permissions::permission.eq(permission))
            .filter(delegated_permissions::scope_type.eq("platform"))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result(conn)
    .await
}
