use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::schema::{role_permission_platform, user_role_platform};

pub async fn platform_user_has_permission(
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

pub async fn assign_platform_role_to_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    platform_role_id: i32,
) -> QueryResult<usize> {
    let new_user_role = (
        user_role_platform::user_id.eq(user_id),
        user_role_platform::platform_role_id.eq(platform_role_id),
    );

    diesel::insert_into(user_role_platform::table)
        .values(&new_user_role)
        .execute(conn)
        .await
}

pub async fn assign_platform_role_to_user_if_missing(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    platform_role_id: i32,
) -> QueryResult<()> {
    let already_assigned = select(exists(
        user_role_platform::table
            .filter(user_role_platform::user_id.eq(user_id))
            .filter(user_role_platform::platform_role_id.eq(platform_role_id)),
    ))
    .get_result::<bool>(conn)
    .await?;

    if !already_assigned {
        assign_platform_role_to_user(conn, user_id, platform_role_id).await?;
    }

    Ok(())
}
