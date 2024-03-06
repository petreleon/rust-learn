use diesel::{dsl::min, prelude::*};

use std::cmp::Ordering;
// Checks if a user has a specific permission on the platform
use crate::db::schema::*;

pub fn user_permission_platform_request(
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
pub fn user_permission_organization_request(
    conn: &mut PgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {

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


pub fn user_hierarchy_compare_platform(
    conn: &mut PgConnection,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {

    let user1_max_level = role_platform_hierarchy::table
        .inner_join(user_role_platform::table.on(role_platform_hierarchy::role_id.eq(user_role_platform::role_id)))
        .filter(user_role_platform::user_id.eq(user1_id))
        .select(min(role_platform_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    let user2_max_level = role_platform_hierarchy::table
        .inner_join(user_role_platform::table.on(role_platform_hierarchy::role_id.eq(user_role_platform::role_id)))
        .filter(user_role_platform::user_id.eq(user2_id))
        .select(min(role_platform_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    match (user1_max_level, user2_max_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater), // User with a role is considered 'higher'
        (None, Some(_)) => Ok(Ordering::Less),
    }
}

pub fn user_hierarchy_compare_organization(
    conn: &mut PgConnection,
    organization_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {

    let user1_max_level = role_organization_hierarchy::table
        .inner_join(user_role_organization::table.on(role_organization_hierarchy::role_id.eq(user_role_organization::role_id)))
        .filter(user_role_organization::user_id.eq(user1_id))
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(min(role_organization_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    let user2_max_level = role_organization_hierarchy::table
        .inner_join(user_role_organization::table.on(role_organization_hierarchy::role_id.eq(user_role_organization::role_id)))
        .filter(user_role_organization::user_id.eq(user2_id))
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(min(role_organization_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    match (user1_max_level, user2_max_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater), 
        (None, Some(_)) => Ok(Ordering::Less),
    }
}
