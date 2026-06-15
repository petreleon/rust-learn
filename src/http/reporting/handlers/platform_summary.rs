use std::sync::Arc;

use actix_web::web;

use crate::application::reporting::platform_summary::PlatformSummaryUseCase;
use crate::http::errors::ApiError;
use crate::http::reporting::dto::{
    csv_download, platform_summary_csv, CsvDownload, PlatformSummaryResponse,
};
use crate::http::reporting::errors::{platform_summary_error, ReportOperation};

pub async fn get_platform_summary(
    summary: web::Data<Arc<dyn PlatformSummaryUseCase>>,
) -> Result<web::Json<PlatformSummaryResponse>, ApiError> {
    summary
        .load_platform_summary()
        .await
        .map(PlatformSummaryResponse::from)
        .map(web::Json)
        .map_err(|error| platform_summary_error(error, ReportOperation::Load))
}

pub async fn export_platform_summary(
    summary: web::Data<Arc<dyn PlatformSummaryUseCase>>,
) -> Result<CsvDownload, ApiError> {
    summary
        .load_platform_summary()
        .await
        .map(PlatformSummaryResponse::from)
        .map(|response| csv_download("platform-summary.csv", platform_summary_csv(&response)))
        .map_err(|error| platform_summary_error(error, ReportOperation::Export))
}
