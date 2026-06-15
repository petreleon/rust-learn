use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::models::user::User;
use crate::infra::postgres::schema::{
    platform_roles, role_permission_platform, user_role_platform, users,
};

pub(super) async fn find_user(conn: &mut AsyncPgConnection, user_id: i32) -> QueryResult<User> {
    users::table.find(user_id).first(conn).await
}

pub(super) async fn list_platform_roles(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Vec<String>> {
    user_role_platform::table
        .inner_join(
            platform_roles::table
                .on(user_role_platform::platform_role_id.eq(platform_roles::id.nullable())),
        )
        .filter(user_role_platform::user_id.eq(user_id))
        .select(platform_roles::name)
        .load(conn)
        .await
}

pub(super) async fn list_platform_permissions(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Vec<String>> {
    user_role_platform::table
        .inner_join(role_permission_platform::table.on(
            user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
        ))
        .filter(user_role_platform::user_id.eq(user_id))
        .select(role_permission_platform::permission)
        .load(conn)
        .await
}
