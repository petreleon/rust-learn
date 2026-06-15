use diesel_async::AsyncPgConnection;

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardOperatorPermissionsOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::organizations::organization_dashboard_mappers::map_dashboard_error;
use crate::infra::postgres::organizations::organization_permission_checks::{
    can_platform_or_organization_permission, has_any_active_organization_delegation,
    has_any_organization_role,
};

pub async fn can_view_dashboard(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, OrganizationDashboardError> {
    if can_organization_operator(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Ok(true);
    }

    if has_any_organization_role(conn, actor_user_id, organization_id)
        .await
        .map_err(map_dashboard_error)?
    {
        return Ok(true);
    }

    has_any_active_organization_delegation(conn, actor_user_id, organization_id)
        .await
        .map_err(map_dashboard_error)
}

pub async fn load_operator_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardError> {
    let can_view_members = can_organization_operator(
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
        can_view_reports: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORG_REWARD_REPORTS,
        )
        .await?,
        can_view_teacher_applications: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORG_TEACHER_APPLICATIONS,
        )
        .await?,
        can_nominate_teachers: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
        )
        .await?,
        can_manage_wallets: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_WALLETS,
        )
        .await?,
        can_manage_reward_budget: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_REWARD_BUDGET,
        )
        .await?,
    })
}

async fn can_organization_operator(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, OrganizationDashboardError> {
    can_platform_or_organization_permission(conn, actor_user_id, organization_id, permission)
        .await
        .map_err(map_dashboard_error)
}
