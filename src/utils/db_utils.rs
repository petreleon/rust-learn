use diesel::{dsl::{any, count}, prelude::*, sql_types::{Int4, Integer}};
use diesel::sql_types::Nullable;

use crate::db::schema::role_permission_organization; 

// Checks if a user has a specific permission on the platform
use crate::db::schema::*;
use diesel::prelude::*;

use diesel::prelude::*;

use diesel::prelude::*;
use crate::utils::db_utils::role_permission_platform::star;
fn user_permission_platform_request(
    conn: &mut PgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let has_permission = diesel::select(diesel::dsl::exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(user_role_platform::role_id.eq(role_permission_platform::role_id)))
            .filter(user_role_platform::user_id.eq(user_id))
            .filter(role_permission_platform::permission.eq(permission))
    ))
    .get_result(conn)?;

    Ok(has_permission)
}


// Checks if a user has a specific permission in an organization
fn user_permission_organization_request(
    conn: &mut PgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    use crate::db::schema::{user_role_organization, role_permission_organization, roles};

    let has_permission = diesel::select(diesel::dsl::exists(
        user_role_organization::table
            .inner_join(roles::table.on(user_role_organization::role_id.eq(roles::id)))
            .inner_join(role_permission_organization::table.on(roles::id.eq(role_permission_organization::role_id)))
            .filter(user_role_organization::user_id.eq(user_id))
            .filter(user_role_organization::organization_id.eq(organization_id))
            .filter(role_permission_organization::permission.eq(permission))
    ))
    .get_result(conn)?;

    Ok(has_permission)
}
