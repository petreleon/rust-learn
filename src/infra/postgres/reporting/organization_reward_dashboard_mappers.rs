use crate::application::reporting::organization_reward_dashboard::OrganizationRewardDashboardError;

pub(super) fn map_diesel_error(error: diesel::result::Error) -> OrganizationRewardDashboardError {
    match error {
        diesel::result::Error::NotFound => OrganizationRewardDashboardError::NotFound,
        other => OrganizationRewardDashboardError::Database(other.to_string()),
    }
}
