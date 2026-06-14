use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::role_permission_platform;

pub async fn assign_platform_permission_to_role(
    conn: &mut AsyncPgConnection,
    platform_role_id: i32,
    permission: &str,
) -> QueryResult<usize> {
    let permission_exists = select(exists(
        role_permission_platform::table
            .filter(
                role_permission_platform::platform_role_id
                    .nullable()
                    .eq(Some(platform_role_id)),
            )
            .filter(role_permission_platform::permission.eq(permission)),
    ))
    .get_result(conn)
    .await?;

    if permission_exists {
        return Ok(0);
    }

    let new_permission = NewPlatformRolePermission {
        platform_role_id: Some(platform_role_id),
        permission,
    };

    diesel::insert_into(role_permission_platform::table)
        .values(&new_permission)
        .execute(conn)
        .await
}

#[derive(Insertable)]
#[diesel(table_name = role_permission_platform)]
struct NewPlatformRolePermission<'a> {
    platform_role_id: Option<i32>,
    permission: &'a str,
}
