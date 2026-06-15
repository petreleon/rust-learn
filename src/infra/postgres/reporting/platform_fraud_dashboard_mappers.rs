use crate::application::reporting::platform_fraud_dashboard::PlatformFraudDashboardError;

pub(super) fn map_diesel_error(error: diesel::result::Error) -> PlatformFraudDashboardError {
    PlatformFraudDashboardError::Database(error.to_string())
}
