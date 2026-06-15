use crate::application::reporting::platform_csv_exports::PlatformCsvExportError;
use crate::application::reporting::platform_fraud_dashboard::PlatformFraudDashboardError;
use crate::application::reporting::platform_reward_dashboard::PlatformRewardDashboardError;
use crate::application::reporting::platform_summary::PlatformSummaryError;
use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationError;
use crate::http::errors::ApiError;
use crate::http::reporting::errors::ReportOperation;

pub(in crate::http::reporting) fn platform_summary_error(
    error: PlatformSummaryError,
    operation: ReportOperation,
) -> ApiError {
    match error {
        PlatformSummaryError::Connection(_) => super::db_connection_failed(),
        PlatformSummaryError::Database(error) => super::logged_internal(
            operation,
            "scope=platform",
            format!("Failed to {} platform report", operation.as_event()),
            error,
        ),
    }
}

pub(in crate::http::reporting) fn platform_fraud_dashboard_error(
    error: PlatformFraudDashboardError,
    operation: ReportOperation,
) -> ApiError {
    platform_dashboard_error(
        error.into(),
        operation,
        "fraud_dashboard",
        "fraud dashboard",
    )
}

pub(in crate::http::reporting) fn platform_reward_dashboard_error(
    error: PlatformRewardDashboardError,
    operation: ReportOperation,
) -> ApiError {
    platform_dashboard_error(
        error.into(),
        operation,
        "reward_dashboard",
        "reward dashboard",
    )
}

pub(in crate::http::reporting) fn platform_wallet_reconciliation_error(
    error: PlatformWalletReconciliationError,
) -> ApiError {
    match error {
        PlatformWalletReconciliationError::Connection(_) => super::db_connection_failed(),
        PlatformWalletReconciliationError::Database(error) => super::logged_internal(
            ReportOperation::Load,
            "scope=platform report=wallet_reconciliation",
            "Failed to load wallet reconciliation",
            error,
        ),
    }
}

pub(in crate::http::reporting) fn platform_csv_export_error(
    error: PlatformCsvExportError,
    report: &'static str,
    label: &'static str,
) -> ApiError {
    match error {
        PlatformCsvExportError::Connection(_) => super::db_connection_failed(),
        PlatformCsvExportError::Database(error) => super::logged_internal(
            ReportOperation::Export,
            &format!("scope=platform report={report}"),
            format!("Failed to export {label}"),
            error,
        ),
    }
}

fn platform_dashboard_error(
    error: PlatformDashboardError,
    operation: ReportOperation,
    report: &'static str,
    label: &'static str,
) -> ApiError {
    match error {
        PlatformDashboardError::Connection => super::db_connection_failed(),
        PlatformDashboardError::Database(error) => super::logged_internal(
            operation,
            &format!("scope=platform report={report}"),
            format!("Failed to {} {label}", operation.as_event()),
            error,
        ),
    }
}

enum PlatformDashboardError {
    Connection,
    Database(String),
}

impl From<PlatformFraudDashboardError> for PlatformDashboardError {
    fn from(error: PlatformFraudDashboardError) -> Self {
        match error {
            PlatformFraudDashboardError::Connection(_) => Self::Connection,
            PlatformFraudDashboardError::Database(error) => Self::Database(error),
        }
    }
}

impl From<PlatformRewardDashboardError> for PlatformDashboardError {
    fn from(error: PlatformRewardDashboardError) -> Self {
        match error {
            PlatformRewardDashboardError::Connection(_) => Self::Connection,
            PlatformRewardDashboardError::Database(error) => Self::Database(error),
        }
    }
}
