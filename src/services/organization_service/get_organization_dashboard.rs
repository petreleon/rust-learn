use crate::config::constants::permissions::Permissions;
use crate::db::schema::organizations;
use crate::models::organization::Organization;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use super::attach_member_permissions::user_has_platform_or_organization_permission;
use super::dashboard_types_and_reads::{
    OrganizationDashboardError, OrganizationDashboardOperatorPermissions,
};
use super::log_organization_member_event::{
    organization_dashboard_course_summary, organization_dashboard_member_summary,
    organization_dashboard_teacher_application_summary,
};
use super::organization_dashboard_alerts::organization_dashboard_alerts;
use super::organization_dashboard_reward_summary::{
    gated_course_summary, gated_member_summary, gated_reward_summary,
    gated_teacher_application_summary, gated_wallet_summary, organization_dashboard_reward_summary,
    organization_dashboard_wallet_summary,
};
use super::support::{
    OrganizationDashboardHealth, OrganizationDashboardOrganization, OrganizationDashboardResponse,
};
use super::user_has_organization_dashboard_access::user_has_organization_dashboard_access;

pub async fn get_organization_dashboard(
    conn: &mut diesel_async::AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationDashboardResponse, OrganizationDashboardError> {
    let organization = organizations::table
        .find(organization_id)
        .first::<Organization>(conn)
        .await
        .map_err(OrganizationDashboardError::from)?;

    let can_view_dashboard =
        user_has_organization_dashboard_access(conn, actor_user_id, organization_id)
            .await
            .map_err(OrganizationDashboardError::from)?;
    if !can_view_dashboard {
        return Err(OrganizationDashboardError::PermissionDenied);
    }

    let can_view_members = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_view_courses = can_view_members;
    let can_view_reports = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORG_REWARD_REPORTS,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_view_teacher_applications = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORG_TEACHER_APPLICATIONS,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_nominate_teachers = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_manage_wallets = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::MANAGE_ORG_WALLETS,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;
    let can_manage_reward_budget = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::MANAGE_ORG_REWARD_BUDGET,
    )
    .await
    .map_err(OrganizationDashboardError::from)?;

    let operator_permissions = OrganizationDashboardOperatorPermissions {
        can_view_dashboard,
        can_view_members,
        can_view_courses,
        can_view_reports,
        can_view_teacher_applications,
        can_nominate_teachers,
        can_manage_wallets,
        can_manage_reward_budget,
    };

    let members = if can_view_members {
        organization_dashboard_member_summary(conn, organization_id).await?
    } else {
        gated_member_summary()
    };
    let courses = if can_view_courses {
        organization_dashboard_course_summary(conn, organization_id).await?
    } else {
        gated_course_summary()
    };
    let teacher_applications = if can_view_teacher_applications || can_nominate_teachers {
        organization_dashboard_teacher_application_summary(conn, organization_id).await?
    } else {
        gated_teacher_application_summary()
    };
    let rewards = if can_view_reports {
        organization_dashboard_reward_summary(conn, organization_id).await?
    } else {
        gated_reward_summary()
    };
    let wallet = if can_view_reports || can_manage_wallets || can_manage_reward_budget {
        organization_dashboard_wallet_summary(conn, organization_id).await?
    } else {
        gated_wallet_summary()
    };

    let alerts = organization_dashboard_alerts(
        organization_id,
        &courses,
        &teacher_applications,
        &rewards,
        &wallet,
        &operator_permissions,
    );
    let health = OrganizationDashboardHealth {
        status: if alerts.iter().any(|alert| alert.severity == "warning") {
            "attention".to_string()
        } else {
            "active".to_string()
        },
        alert_count: alerts.len(),
    };

    Ok(OrganizationDashboardResponse {
        organization: OrganizationDashboardOrganization {
            id: organization.id,
            name: organization.name,
        },
        health,
        members,
        courses,
        teacher_applications,
        rewards,
        wallet,
        operator_permissions,
        alerts,
    })
}

pub struct CreateOrganizationDto {
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
    pub course_ids: Option<Vec<i32>>,
}
