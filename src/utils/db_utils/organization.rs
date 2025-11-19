use diesel::{dsl::min, prelude::*};
use std::cmp::Ordering;
use crate::models::user_role_organization::UserRoleOrganization;
use crate::models::role_organization_hierarchy::RoleOrganizationHierarchy;

/// Checks if a user has a specific permission in an organization
pub fn user_permission_organization_request(
    conn: &mut PgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    UserRoleOrganization::has_permission(conn, user_id, organization_id, permission)
}

/// Compares the hierarchy of two users in an organization
pub fn user_hierarchy_compare_organization(
    conn: &mut PgConnection,
    organization_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_max_level = RoleOrganizationHierarchy::get_min_level(conn, user1_id, organization_id)?;
    let user2_max_level = RoleOrganizationHierarchy::get_min_level(conn, user2_id, organization_id)?;

    match (user1_max_level, user2_max_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater),
        (None, Some(_)) => Ok(Ordering::Less),
    }
}
