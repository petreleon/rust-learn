use diesel::{dsl::min, prelude::*};
use std::cmp::Ordering;
use crate::db::schema::*;

/// Checks if a user has a specific permission in an organization
pub fn user_permission_organization_request(
    conn: &mut PgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let has_permission = diesel::select(diesel::dsl::exists(
        user_role_organization::table
            .inner_join(organization_roles::table.on(
                user_role_organization::organization_role_id.eq(organization_roles::id.nullable())
            ))
            .inner_join(role_permission_organization::table.on(
                organization_roles::id.nullable().eq(role_permission_organization::organization_role_id)
            ))
            .filter(user_role_organization::user_id.eq(user_id))
            .filter(user_role_organization::organization_id.eq(organization_id))
            .filter(role_permission_organization::permission.eq(permission))
    ))
    .get_result(conn)?;

    Ok(has_permission)
}

/// Compares the hierarchy of two users in an organization
pub fn user_hierarchy_compare_organization(
    conn: &mut PgConnection,
    organization_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_max_level = role_organization_hierarchy::table
        .inner_join(user_role_organization::table.on(
            role_organization_hierarchy::organization_role_id.eq(user_role_organization::organization_role_id)
        ))
        .filter(user_role_organization::user_id.eq(user1_id))
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(min(role_organization_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    let user2_max_level = role_organization_hierarchy::table
        .inner_join(user_role_organization::table.on(
            role_organization_hierarchy::organization_role_id.eq(user_role_organization::organization_role_id)
        ))
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
