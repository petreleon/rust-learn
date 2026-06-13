use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberOperatorPermissionsOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    delegated_permissions, role_permission_organization, user_role_organization,
};
use crate::infra::postgres::organizations::organization_member_builders::OrganizationMemberBuilder;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;

pub async fn can_view_organization_members(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, OrganizationMemberListError> {
    user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await
}

pub async fn build_member_operator_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationMemberOperatorPermissionsOutput, OrganizationMemberListError> {
    Ok(OrganizationMemberOperatorPermissionsOutput {
        can_view_members: can_view_organization_members(conn, actor_user_id, organization_id)
            .await?,
        can_invite_members: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::INVITE_USER_TO_ORGANIZATION,
        )
        .await?,
        can_manage_members: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_MEMBERS,
        )
        .await?,
        can_assign_roles: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::ASSIGN_ROLES_TO_ORG_USERS,
        )
        .await?,
        can_manage_settings: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_SETTINGS,
        )
        .await?,
    })
}

pub(super) async fn attach_member_permissions(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    members: &mut BTreeMap<i32, OrganizationMemberBuilder>,
) -> Result<(), OrganizationMemberListError> {
    if members.is_empty() {
        return Ok(());
    }

    let rows = user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::organization_id.eq(organization_id))
        .filter(
            role_permission_organization::organization_id
                .is_null()
                .or(role_permission_organization::organization_id.eq(Some(organization_id))),
        )
        .select((
            user_role_organization::user_id,
            role_permission_organization::permission,
        ))
        .load::<(Option<i32>, String)>(conn)
        .await
        .map_err(map_member_error)?;

    for (user_id, permission) in rows {
        if let Some(member) = user_id.and_then(|id| members.get_mut(&id)) {
            member.direct_permissions.insert(permission);
        }
    }

    Ok(())
}

pub(super) async fn attach_member_delegations(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    members: &mut BTreeMap<i32, OrganizationMemberBuilder>,
) -> Result<(), OrganizationMemberListError> {
    if members.is_empty() {
        return Ok(());
    }

    let now: DateTime<Utc> = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::scope_type.eq("organization"))
        .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
        .filter(delegated_permissions::course_id.is_null())
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select((
            delegated_permissions::grantee_user_id,
            delegated_permissions::permission,
        ))
        .load::<(i32, String)>(conn)
        .await
        .map_err(map_member_error)?;

    for (user_id, permission) in rows {
        if let Some(member) = members.get_mut(&user_id) {
            member.delegated_permissions.insert(permission);
        }
    }

    Ok(())
}

async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, OrganizationMemberListError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission_name)
        .await
        .map_err(map_member_error)?
    {
        return Ok(true);
    }

    user_permission_organization_request(conn, actor_user_id, organization_id, &permission_name)
        .await
        .map_err(map_member_error)
}

fn map_member_error(error: diesel::result::Error) -> OrganizationMemberListError {
    match error {
        diesel::result::Error::NotFound => OrganizationMemberListError::NotFound,
        other => OrganizationMemberListError::Database(other.to_string()),
    }
}
