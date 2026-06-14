use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{
    role_permission_organization, role_permission_platform, user_role_organization,
    user_role_platform,
};

pub async fn list_platform_user_ids_with_permission(
    conn: &mut AsyncPgConnection,
    permission: &str,
) -> QueryResult<Vec<i32>> {
    user_role_platform::table
        .inner_join(
            role_permission_platform::table.on(user_role_platform::platform_role_id
                .nullable()
                .eq(role_permission_platform::platform_role_id)),
        )
        .filter(role_permission_platform::permission.eq(permission))
        .select(user_role_platform::user_id.assume_not_null())
        .distinct()
        .load::<i32>(conn)
        .await
}

pub async fn list_organization_user_ids_with_permission(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    permission: &str,
) -> QueryResult<Vec<i32>> {
    user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::organization_id.eq(organization_id))
        .filter(role_permission_organization::permission.eq(permission))
        .select(user_role_organization::user_id.assume_not_null())
        .distinct()
        .load::<i32>(conn)
        .await
}
