use std::sync::Arc;

use actix_web::web;

use crate::application::reporting::platform_reward_dashboard::PlatformRewardDashboardUseCase;
use crate::http::errors::ApiError;
use crate::http::reporting::dto::{
    csv_download, platform_reward_dashboard_csv, CsvDownload, PlatformRewardDashboardResponse,
};
use crate::http::reporting::errors::{platform_reward_dashboard_error, ReportOperation};

pub async fn get_platform_reward_dashboard(
    dashboard: web::Data<Arc<dyn PlatformRewardDashboardUseCase>>,
) -> Result<web::Json<PlatformRewardDashboardResponse>, ApiError> {
    dashboard
        .load_platform_reward_dashboard()
        .await
        .map(PlatformRewardDashboardResponse::from)
        .map(web::Json)
        .map_err(|error| platform_reward_dashboard_error(error, ReportOperation::Load))
}

pub async fn export_platform_reward_dashboard(
    dashboard: web::Data<Arc<dyn PlatformRewardDashboardUseCase>>,
) -> Result<CsvDownload, ApiError> {
    dashboard
        .load_platform_reward_dashboard()
        .await
        .map(PlatformRewardDashboardResponse::from)
        .map(|response| {
            csv_download(
                "platform-reward-dashboard.csv",
                platform_reward_dashboard_csv(&response),
            )
        })
        .map_err(|error| platform_reward_dashboard_error(error, ReportOperation::Export))
}
