use crate::application::organizations::get_organization_dashboard::alerts::organization_dashboard_alerts;
use crate::application::organizations::get_organization_dashboard::gated::{
    gated_course_summary, gated_member_summary, gated_reward_summary,
    gated_teacher_application_summary, gated_wallet_summary,
};
use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardHealthOutput, OrganizationDashboardOutput,
    OrganizationDashboardQuery, OrganizationDashboardStore,
};

pub async fn get_organization_dashboard(
    store: &mut impl OrganizationDashboardStore,
    query: OrganizationDashboardQuery,
) -> Result<OrganizationDashboardOutput, OrganizationDashboardError> {
    let organization = store.load_organization(query.organization_id).await?;
    if !store
        .can_view_dashboard(query.actor_user_id, query.organization_id)
        .await?
    {
        return Err(OrganizationDashboardError::PermissionDenied);
    }

    let permissions = store
        .load_operator_permissions(query.actor_user_id, query.organization_id)
        .await?;
    let members = if permissions.can_view_members {
        store.load_member_summary(query.organization_id).await?
    } else {
        gated_member_summary()
    };
    let courses = if permissions.can_view_courses {
        store.load_course_summary(query.organization_id).await?
    } else {
        gated_course_summary()
    };
    let teacher_applications =
        if permissions.can_view_teacher_applications || permissions.can_nominate_teachers {
            store
                .load_teacher_application_summary(query.organization_id)
                .await?
        } else {
            gated_teacher_application_summary()
        };
    let rewards = if permissions.can_view_reports {
        store.load_reward_summary(query.organization_id).await?
    } else {
        gated_reward_summary()
    };
    let wallet = if permissions.can_view_reports
        || permissions.can_manage_wallets
        || permissions.can_manage_reward_budget
    {
        store.load_wallet_summary(query.organization_id).await?
    } else {
        gated_wallet_summary()
    };

    let alerts = organization_dashboard_alerts(
        query.organization_id,
        &courses,
        &teacher_applications,
        &rewards,
        &wallet,
        &permissions,
    );
    let health = OrganizationDashboardHealthOutput {
        status: if alerts.iter().any(|alert| alert.severity == "warning") {
            "attention".to_string()
        } else {
            "active".to_string()
        },
        alert_count: alerts.len(),
    };

    Ok(OrganizationDashboardOutput {
        organization,
        health,
        members,
        courses,
        teacher_applications,
        rewards,
        wallet,
        operator_permissions: permissions,
        alerts,
    })
}

#[cfg(test)]
mod tests;
