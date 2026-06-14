use crate::config::constants::permissions::Permissions;
use crate::db::schema::{delegated_permissions, user_role_organization};
use chrono::{DateTime, Utc};
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::collections::BTreeSet;

use super::attach_member_permissions::user_has_platform_or_organization_permission;
use super::dashboard_types_and_reads::OrganizationMemberBuilder;
use super::support::{OrganizationMemberListItem, OrganizationMemberListQuery};

pub(super) async fn user_has_organization_dashboard_access(
    conn: &mut diesel_async::AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
) -> QueryResult<bool> {
    if user_has_platform_or_organization_permission(
        conn,
        user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Ok(true);
    }

    let has_role = select(exists(
        user_role_organization::table
            .filter(user_role_organization::user_id.eq(Some(user_id)))
            .filter(user_role_organization::organization_id.eq(Some(organization_id))),
    ))
    .get_result::<bool>(conn)
    .await?;
    if has_role {
        return Ok(true);
    }

    let now: DateTime<Utc> = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(user_id))
            .filter(delegated_permissions::scope_type.eq("organization"))
            .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result::<bool>(conn)
    .await
}

pub(super) fn member_matches_query(
    member: &OrganizationMemberListItem,
    query: &OrganizationMemberListQuery,
) -> bool {
    let matches_search = query
        .search
        .as_deref()
        .map(|search| {
            let normalized = search.to_lowercase();
            member.name.to_lowercase().contains(&normalized)
                || member.email.to_lowercase().contains(&normalized)
                || member
                    .roles
                    .iter()
                    .any(|role| role.to_lowercase().contains(&normalized))
                || member
                    .effective_permissions
                    .iter()
                    .any(|permission| permission.to_lowercase().contains(&normalized))
        })
        .unwrap_or(true);

    let matches_role = query
        .role
        .as_deref()
        .map(|role| {
            member
                .roles
                .iter()
                .any(|member_role| member_role.eq_ignore_ascii_case(role))
        })
        .unwrap_or(true);

    let matches_permission = query
        .permission
        .as_deref()
        .map(|permission| {
            member
                .effective_permissions
                .iter()
                .any(|member_permission| member_permission.eq_ignore_ascii_case(permission))
        })
        .unwrap_or(true);

    matches_search && matches_role && matches_permission
}

pub(super) fn normalize_query_value(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub(super) fn sorted_vec(values: BTreeSet<String>) -> Vec<String> {
    values.into_iter().collect()
}

impl From<OrganizationMemberBuilder> for OrganizationMemberListItem {
    fn from(member: OrganizationMemberBuilder) -> Self {
        let direct_permissions = sorted_vec(member.direct_permissions);
        let delegated_permissions = sorted_vec(member.delegated_permissions);
        let effective_permissions = direct_permissions
            .iter()
            .chain(delegated_permissions.iter())
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let direct_permission_count = direct_permissions.len();
        let delegated_permission_count = delegated_permissions.len();
        let effective_permission_count = effective_permissions.len();

        Self {
            id: member.id,
            name: member.name,
            email: member.email,
            email_verified: member.email_verified,
            kyc_verified: member.kyc_verified,
            joined_at: member.joined_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            roles: sorted_vec(member.roles),
            direct_permissions,
            delegated_permissions,
            effective_permissions,
            direct_permission_count,
            delegated_permission_count,
            effective_permission_count,
        }
    }
}
