use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{
    delegated_permissions, role_permission_course, role_permission_platform, user_role_course,
    user_role_platform,
};

pub async fn has_course_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> diesel::QueryResult<bool> {
    if has_course_role_permission(conn, actor_user_id, course_id, permission).await? {
        return Ok(true);
    }

    has_active_course_delegation(conn, actor_user_id, course_id, permission).await
}

pub async fn has_platform_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> diesel::QueryResult<bool> {
    if has_platform_role_permission(conn, actor_user_id, permission).await? {
        return Ok(true);
    }

    has_active_platform_delegation(conn, actor_user_id, permission).await
}

async fn has_course_role_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> diesel::QueryResult<bool> {
    select(
        exists(
            user_role_course::table
                .inner_join(role_permission_course::table.on(
                    user_role_course::course_role_id.eq(role_permission_course::course_role_id),
                ))
                .filter(user_role_course::user_id.eq(actor_user_id))
                .filter(user_role_course::course_id.eq(course_id))
                .filter(role_permission_course::permission.eq(permission)),
        ),
    )
    .get_result(conn)
    .await
}

async fn has_platform_role_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> diesel::QueryResult<bool> {
    select(exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(
                user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
            ))
            .filter(user_role_platform::user_id.eq(actor_user_id))
            .filter(role_permission_platform::permission.eq(permission)),
    ))
    .get_result(conn)
    .await
}

async fn has_active_course_delegation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> diesel::QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::permission.eq(permission))
            .filter(delegated_permissions::scope_type.eq("course"))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.eq(Some(course_id)))
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

async fn has_active_platform_delegation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> diesel::QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
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
