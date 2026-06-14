use crate::application::organizations::get_organization_dashboard::OrganizationDashboardError;
use crate::application::organizations::list_organization_members::OrganizationMemberListError;
use crate::application::reporting::organization_reward_dashboard::OrganizationRewardDashboardError;

pub fn map_dashboard_error(error: diesel::result::Error) -> OrganizationDashboardError {
    match error {
        diesel::result::Error::NotFound => OrganizationDashboardError::NotFound,
        other => OrganizationDashboardError::Database(other.to_string()),
    }
}

pub fn map_member_list_error(error: OrganizationMemberListError) -> OrganizationDashboardError {
    match error {
        OrganizationMemberListError::PermissionDenied => {
            OrganizationDashboardError::PermissionDenied
        }
        OrganizationMemberListError::NotFound => OrganizationDashboardError::NotFound,
        OrganizationMemberListError::Connection(message) => {
            OrganizationDashboardError::Connection(message)
        }
        OrganizationMemberListError::Database(message) => {
            OrganizationDashboardError::Database(message)
        }
    }
}

pub fn map_reward_dashboard_error(
    error: OrganizationRewardDashboardError,
) -> OrganizationDashboardError {
    match error {
        OrganizationRewardDashboardError::NotFound => OrganizationDashboardError::NotFound,
        OrganizationRewardDashboardError::Connection(message)
        | OrganizationRewardDashboardError::Database(message) => {
            OrganizationDashboardError::Reporting(message)
        }
    }
}
