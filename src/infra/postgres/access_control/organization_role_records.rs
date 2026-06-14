use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{organization_roles, role_permission_organization, user_role_organization};

pub async fn organization_user_has_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    select(exists(
        user_role_organization::table
            .inner_join(organization_roles::table.on(
                user_role_organization::organization_role_id.eq(organization_roles::id.nullable()),
            ))
            .inner_join(
                role_permission_organization::table.on(organization_roles::id
                    .nullable()
                    .eq(role_permission_organization::organization_role_id)),
            )
            .filter(user_role_organization::user_id.eq(user_id))
            .filter(user_role_organization::organization_id.eq(organization_id))
            .filter(role_permission_organization::permission.eq(permission)),
    ))
    .get_result(conn)
    .await
}

pub async fn assign_organization_role_to_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    organization_role_id: i32,
) -> QueryResult<usize> {
    let new_user_role = (
        user_role_organization::user_id.eq(user_id),
        user_role_organization::organization_role_id.eq(organization_role_id),
        user_role_organization::organization_id.eq(organization_id),
    );

    diesel::insert_into(user_role_organization::table)
        .values(&new_user_role)
        .execute(conn)
        .await
}
