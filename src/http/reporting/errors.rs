use actix_web::http::StatusCode;

use crate::http::errors::ApiError;

mod organization;
mod platform;

pub(super) use organization::{organization_reward_dashboard_error, organization_summary_error};
pub(super) use platform::{
    platform_csv_export_error, platform_fraud_dashboard_error, platform_reward_dashboard_error,
    platform_summary_error, platform_wallet_reconciliation_error,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ReportOperation {
    Load,
    Export,
}

impl ReportOperation {
    fn as_event(self) -> &'static str {
        match self {
            Self::Load => "load",
            Self::Export => "export",
        }
    }
}

pub(in crate::http::reporting) fn db_connection_failed() -> ApiError {
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "db_connection_failed",
        "Failed to get DB connection",
    )
}

pub(in crate::http::reporting) fn organization_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "organization_not_found",
        "Organization not found",
    )
}

pub(in crate::http::reporting) fn logged_internal(
    operation: ReportOperation,
    context: &str,
    message: impl Into<String>,
    error: String,
) -> ApiError {
    log::error!(
        "event=report_{}_failed {} error={}",
        operation.as_event(),
        context,
        error
    );
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "reporting_request_failed",
        message,
    )
}

#[cfg(test)]
mod tests;
