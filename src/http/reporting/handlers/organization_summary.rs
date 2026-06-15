use std::sync::Arc;

use actix_web::web;

use crate::application::reporting::organization_summary::OrganizationSummaryUseCase;
use crate::http::errors::ApiError;
use crate::http::reporting::dto::{
    csv_download, organization_summary_csv, CsvDownload, OrganizationSummaryResponse,
};
use crate::http::reporting::errors::{organization_summary_error, ReportOperation};

pub async fn get_organization_summary(
    path: web::Path<i32>,
    summaries: web::Data<Arc<dyn OrganizationSummaryUseCase>>,
) -> Result<web::Json<OrganizationSummaryResponse>, ApiError> {
    let organization_id = path.into_inner();
    summaries
        .load_organization_summary(organization_id)
        .await
        .map(OrganizationSummaryResponse::from)
        .map(web::Json)
        .map_err(|error| organization_summary_error(error, ReportOperation::Load, organization_id))
}

pub async fn export_organization_summary(
    path: web::Path<i32>,
    summaries: web::Data<Arc<dyn OrganizationSummaryUseCase>>,
) -> Result<CsvDownload, ApiError> {
    let organization_id = path.into_inner();
    summaries
        .load_organization_summary(organization_id)
        .await
        .map(OrganizationSummaryResponse::from)
        .map(|response| {
            csv_download(
                format!("organization-{}-summary.csv", organization_id),
                organization_summary_csv(&response),
            )
        })
        .map_err(|error| {
            organization_summary_error(error, ReportOperation::Export, organization_id)
        })
}
