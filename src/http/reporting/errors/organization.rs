use crate::application::reporting::organization_reward_dashboard::OrganizationRewardDashboardError;
use crate::application::reporting::organization_summary::OrganizationSummaryError;
use crate::http::errors::ApiError;
use crate::http::reporting::errors::ReportOperation;

pub(in crate::http::reporting) fn organization_summary_error(
    error: OrganizationSummaryError,
    operation: ReportOperation,
    organization_id: i32,
) -> ApiError {
    match error {
        OrganizationSummaryError::NotFound => super::organization_not_found(),
        OrganizationSummaryError::Connection(_) => super::db_connection_failed(),
        OrganizationSummaryError::Database(error) => super::logged_internal(
            operation,
            &format!("scope=organization organization_id={organization_id}"),
            format!("Failed to {} organization report", operation.as_event()),
            error,
        ),
    }
}

pub(in crate::http::reporting) fn organization_reward_dashboard_error(
    error: OrganizationRewardDashboardError,
    operation: ReportOperation,
    organization_id: i32,
) -> ApiError {
    match error {
        OrganizationRewardDashboardError::NotFound => super::organization_not_found(),
        OrganizationRewardDashboardError::Connection(_) => super::db_connection_failed(),
        OrganizationRewardDashboardError::Database(error) => super::logged_internal(
            operation,
            &format!(
                "scope=organization report=reward_dashboard organization_id={organization_id}"
            ),
            format!(
                "Failed to {} organization reward dashboard",
                operation.as_event()
            ),
            error,
        ),
    }
}
