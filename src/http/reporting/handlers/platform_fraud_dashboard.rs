use std::sync::Arc;

use actix_web::web;

use crate::application::reporting::platform_fraud_dashboard::PlatformFraudDashboardUseCase;
use crate::http::errors::ApiError;
use crate::http::reporting::dto::{
    csv_download, platform_fraud_dashboard_csv, CsvDownload, PlatformFraudDashboardResponse,
};
use crate::http::reporting::errors::{platform_fraud_dashboard_error, ReportOperation};

pub async fn get_platform_fraud_dashboard(
    dashboard: web::Data<Arc<dyn PlatformFraudDashboardUseCase>>,
) -> Result<web::Json<PlatformFraudDashboardResponse>, ApiError> {
    dashboard
        .load_platform_fraud_dashboard()
        .await
        .map(PlatformFraudDashboardResponse::from)
        .map(web::Json)
        .map_err(|error| platform_fraud_dashboard_error(error, ReportOperation::Load))
}

pub async fn export_platform_fraud_dashboard(
    dashboard: web::Data<Arc<dyn PlatformFraudDashboardUseCase>>,
) -> Result<CsvDownload, ApiError> {
    dashboard
        .load_platform_fraud_dashboard()
        .await
        .map(PlatformFraudDashboardResponse::from)
        .map(|response| {
            csv_download(
                "platform-fraud-dashboard.csv",
                platform_fraud_dashboard_csv(&response),
            )
        })
        .map_err(|error| platform_fraud_dashboard_error(error, ReportOperation::Export))
}
