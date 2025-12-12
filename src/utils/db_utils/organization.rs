use diesel::{dsl::min, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::cmp::Ordering;
use crate::models::user_role_organization::UserRoleOrganization;
use crate::models::role_organization_hierarchy::RoleOrganizationHierarchy;

/// Checks if a user has a specific permission in an organization
pub async fn user_permission_organization_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    UserRoleOrganization::has_permission(conn, user_id, organization_id, permission).await
}

/// Compares the hierarchy of two users in an organization
pub async fn user_hierarchy_compare_organization(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_max_level = RoleOrganizationHierarchy::get_min_level(conn, user1_id, organization_id).await?;
    let user2_max_level = RoleOrganizationHierarchy::get_min_level(conn, user2_id, organization_id).await?;


    match (user1_max_level, user2_max_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater),
        (None, Some(_)) => Ok(Ordering::Less),
    }
}

pub async fn assign_role_to_user_in_organization(
    conn: &mut AsyncPgConnection,
    assigner_id: i32,
    p_user_id: i32,
    p_organization_id: i32,
    role_name: &str,
) -> QueryResult<usize> {
    
    // 1. Get Assigner's Hierarchy Level
    let assigner_level = RoleOrganizationHierarchy::get_min_level(conn, assigner_id, p_organization_id).await?
        .ok_or(diesel::result::Error::NotFound)?; // Assigner must have a role in the org

    // 2. Get Assignee's (Target User) Hierarchy Level
    let assignee_level_opt = RoleOrganizationHierarchy::get_min_level(conn, p_user_id, p_organization_id).await?;

    // 3. Get Role ID
    let role_id = crate::models::role::OrganizationRole::find_by_name(role_name, conn).await?;

    // 4. Get Target Role's Hierarchy Level
    let target_role_level = RoleOrganizationHierarchy::get_role_level(conn, role_id).await?;

    // 5. Enforce Hierarchy Rules: Lower value means higher rank (0 is highest)
    
    // Rule A: Assigner must be higher rank than the role they are assigning
    if assigner_level >= target_role_level {
        return Err(diesel::result::Error::RollbackTransaction); 
    }

    // Rule B: Assigner must be higher rank than the user they are assigning to (if user already has a role)
    if let Some(assignee_level) = assignee_level_opt {
        if assigner_level >= assignee_level {
             return Err(diesel::result::Error::RollbackTransaction);
        }
    }

    UserRoleOrganization::assign(conn, p_user_id, p_organization_id, role_id).await
}
