use crate::config::constants::permissions::Permissions;
use crate::db::schema::organizations;
use crate::models::organization::Organization;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use super::attach_member_permissions::{
    attach_member_delegations, attach_member_permissions, build_member_operator_permissions,
    user_has_platform_or_organization_permission,
};
use super::organization_dashboard_alerts::build_organization_member_builders;
use super::support::{
    OrganizationMemberListError, OrganizationMemberListItem, OrganizationMemberListOrganization,
    OrganizationMemberListQuery, OrganizationMemberListResponse,
};
use super::user_has_organization_dashboard_access::member_matches_query;

pub async fn list_organization_members(
    conn: &mut diesel_async::AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    query: OrganizationMemberListQuery,
) -> Result<OrganizationMemberListResponse, OrganizationMemberListError> {
    let organization = organizations::table
        .find(organization_id)
        .first::<Organization>(conn)
        .await
        .map_err(OrganizationMemberListError::from)?;

    if !user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Err(OrganizationMemberListError::PermissionDenied);
    }

    let operator_permissions =
        build_member_operator_permissions(conn, actor_user_id, organization_id).await?;
    let mut members = build_organization_member_builders(conn, organization_id).await?;
    attach_member_permissions(conn, organization_id, &mut members).await?;
    attach_member_delegations(conn, organization_id, &mut members).await?;

    let mut items = members
        .into_values()
        .map(OrganizationMemberListItem::from)
        .filter(|member| member_matches_query(member, &query))
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.email.to_lowercase().cmp(&right.email.to_lowercase()))
            .then_with(|| left.id.cmp(&right.id))
    });

    let total = items.len() as i64;
    let members = items
        .into_iter()
        .skip(query.offset as usize)
        .take(query.limit as usize)
        .collect::<Vec<_>>();

    Ok(OrganizationMemberListResponse {
        organization: OrganizationMemberListOrganization {
            id: organization.id,
            name: organization.name,
        },
        members,
        total,
        limit: query.limit,
        offset: query.offset,
        search: query.search,
        role: query.role,
        permission: query.permission,
        operator_permissions,
    })
}
