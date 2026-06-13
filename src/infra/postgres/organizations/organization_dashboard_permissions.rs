use chrono::{DateTime, Utc};
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardOperatorPermissionsOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{delegated_permissions, user_role_organization};
use crate::infra::postgres::organizations::organization_dashboard_mappers::map_dashboard_error;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;

pub async fn can_view_dashboard(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, OrganizationDashboardError> {
    if user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Ok(true);
    }

    if user_has_organization_role(conn, actor_user_id, organization_id).await? {
        return Ok(true);
    }

    user_has_active_organization_delegation(conn, actor_user_id, organization_id).await
}

pub async fn load_operator_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardError> {
    let can_view_members = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?;

    Ok(OrganizationDashboardOperatorPermissionsOutput {
        can_view_dashboard: true,
        can_view_members,
        can_view_courses: can_view_members,
        can_view_reports: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORG_REWARD_REPORTS,
        )
        .await?,
        can_view_teacher_applications: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORG_TEACHER_APPLICATIONS,
        )
        .await?,
        can_nominate_teachers: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
        )
        .await?,
        can_manage_wallets: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_WALLETS,
        )
        .await?,
        can_manage_reward_budget: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_REWARD_BUDGET,
        )
        .await?,
    })
}

async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, OrganizationDashboardError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission_name)
        .await
        .map_err(map_dashboard_error)?
    {
        return Ok(true);
    }

    user_permission_organization_request(conn, actor_user_id, organization_id, &permission_name)
        .await
        .map_err(map_dashboard_error)
}

async fn user_has_organization_role(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, OrganizationDashboardError> {
    select(exists(
        user_role_organization::table
            .filter(user_role_organization::user_id.eq(Some(actor_user_id)))
            .filter(user_role_organization::organization_id.eq(Some(organization_id))),
    ))
    .get_result::<bool>(conn)
    .await
    .map_err(map_dashboard_error)
}

async fn user_has_active_organization_delegation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, OrganizationDashboardError> {
    let now: DateTime<Utc> = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
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
    .map_err(map_dashboard_error)
}
