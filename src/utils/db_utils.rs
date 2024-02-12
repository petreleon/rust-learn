use diesel::{dsl::any, prelude::*, sql_types::{Int4, Integer}};
use diesel::sql_types::Nullable;

use crate::db::schema::role_permission_organization; 

// Checks if a user has a specific permission on the platform
use crate::db::schema::*;
use diesel::prelude::*;

fn user_permission_platform_request(
    conn: &mut PgConnection, user_id: i32, permission: &str
) -> QueryResult<bool> {

    let subquery = user_role_platform::table
        .filter(user_role_platform::user_id.eq(user_id))
        .select(user_role_platform::role_id);

    let has_permission = role_permission_platform::table
        .filter(role_permission_platform::role_id.nullable().eq_any(subquery.into_boxed()))
        .filter(role_permission_platform::permission.eq(permission))
        .select(diesel::dsl::exists(
            role_permission_platform::table
                .inner_join(roles::table.on(role_permission_platform::role_id.eq(roles::id)))
                .filter(role_permission_platform::permission.eq(permission))
                .filter(role_permission_platform::role_id.nullable().eq_any(subquery))
        ))
        .get_result(conn)?;

    Ok(has_permission)
}


// Checks if a user has a specific permission in an organization
fn user_permission_organization_request(
    conn: &mut PgConnection, user_id: i32, organization_id: i32, permission: &str
) -> QueryResult<bool> {
    use crate::db::schema::{roles, role_permission_organization};

    let has_permission = role_permission_organization::table
        .inner_join(roles::table)
        .filter(roles::id.eq(role_permission_organization::role_id.nullable())) // Remove the generic argument
        .filter(role_permission_organization::organization_id.eq(organization_id))
        .filter(role_permission_organization::permission.eq(permission))
        .select(diesel::dsl::exists(
            role_permission_organization::table.filter(role_permission_organization::role_id.eq_any(
                roles::table.select(roles::id).filter(roles::id.eq(user_id)).nullable(),
            )),
        ))
        .get_result(conn)?;

    Ok(has_permission)
}

