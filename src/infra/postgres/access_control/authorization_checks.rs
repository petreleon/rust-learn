use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use std::cmp::Ordering;

use crate::config::constants::permissions::Permissions;
use crate::config::constants::roles::Roles;
use crate::infra::postgres::access_control::{
    hierarchy_records, organization_role_records, permission_assignment_records, permission_checks,
    platform_role_records, role_catalog_store,
};

pub async fn user_permission_platform_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::has_platform_permission(conn, user_id, permission).await
}

pub async fn user_permission_course_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::has_course_permission(conn, user_id, course_id, permission).await
}

pub async fn user_permission_organization_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::has_organization_permission(conn, user_id, organization_id, permission).await
}

pub async fn user_hierarchy_compare_platform(
    conn: &mut AsyncPgConnection,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_top_level = hierarchy_records::platform_min_level_for_user(conn, user1_id).await?;
    let user2_top_level = hierarchy_records::platform_min_level_for_user(conn, user2_id).await?;

    compare_top_levels(user1_top_level, user2_top_level)
}

pub async fn user_hierarchy_compare_organization(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_top_level =
        hierarchy_records::organization_min_level_for_user(conn, user1_id, organization_id).await?;
    let user2_top_level =
        hierarchy_records::organization_min_level_for_user(conn, user2_id, organization_id).await?;

    compare_top_levels(user1_top_level, user2_top_level)
}

pub async fn user_hierarchy_compare_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_top_level =
        hierarchy_records::course_min_level_for_user(conn, user1_id, course_id).await?;
    let user2_top_level =
        hierarchy_records::course_min_level_for_user(conn, user2_id, course_id).await?;

    compare_top_levels(user1_top_level, user2_top_level)
}

pub async fn assign_role_to_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    role: Roles,
) -> QueryResult<usize> {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, &role.to_string()).await?;
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id).await
}

pub async fn assign_permission_to_role_platform(
    conn: &mut AsyncPgConnection,
    role: Roles,
    permission: Permissions,
) -> QueryResult<usize> {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, &role.to_string()).await?;
    permission_assignment_records::assign_platform_permission_to_role(
        conn,
        role_id,
        &permission.to_string(),
    )
    .await
}

pub async fn assign_role_to_user_in_organization(
    conn: &mut AsyncPgConnection,
    assigner_id: i32,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) -> QueryResult<usize> {
    let assigner_level =
        hierarchy_records::organization_min_level_for_user(conn, assigner_id, organization_id)
            .await?
            .ok_or(diesel::result::Error::NotFound)?;
    let assignee_level =
        hierarchy_records::organization_min_level_for_user(conn, user_id, organization_id).await?;
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name).await?;
    let role_level = hierarchy_records::organization_role_level(conn, role_id).await?;

    if assigner_level >= role_level || assignee_level.is_some_and(|level| assigner_level >= level) {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
}

fn compare_top_levels(
    user1_top_level: Option<i32>,
    user2_top_level: Option<i32>,
) -> QueryResult<Ordering> {
    match (user1_top_level, user2_top_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater),
        (None, Some(_)) => Ok(Ordering::Less),
    }
}
