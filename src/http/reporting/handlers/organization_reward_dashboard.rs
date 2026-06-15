use std::collections::HashMap;
use std::sync::Arc;

use actix_web::web;
use chrono::NaiveDate;

use crate::application::reporting::organization_reward_dashboard::OrganizationRewardDashboardUseCase;
use crate::http::errors::ApiError;
use crate::http::reporting::dto::{
    csv_download, organization_reward_dashboard_csv, CsvDownload,
    OrganizationRewardDashboardResponse,
};
use crate::http::reporting::errors::{organization_reward_dashboard_error, ReportOperation};

pub async fn get_organization_reward_dashboard(
    path: web::Path<i32>,
    dashboard: web::Data<Arc<dyn OrganizationRewardDashboardUseCase>>,
    query: web::Query<HashMap<String, String>>,
) -> Result<web::Json<OrganizationRewardDashboardResponse>, ApiError> {
    let organization_id = path.into_inner();
    let (from, to) = date_range(&query);
    dashboard
        .load_organization_reward_dashboard(organization_id, from, to)
        .await
        .map(OrganizationRewardDashboardResponse::from)
        .map(web::Json)
        .map_err(|error| {
            organization_reward_dashboard_error(error, ReportOperation::Load, organization_id)
        })
}

pub async fn export_organization_reward_dashboard(
    path: web::Path<i32>,
    dashboard: web::Data<Arc<dyn OrganizationRewardDashboardUseCase>>,
    query: web::Query<HashMap<String, String>>,
) -> Result<CsvDownload, ApiError> {
    let organization_id = path.into_inner();
    let (from, to) = date_range(&query);
    dashboard
        .load_organization_reward_dashboard(organization_id, from, to)
        .await
        .map(OrganizationRewardDashboardResponse::from)
        .map(|response| {
            csv_download(
                format!("organization-{}-reward-dashboard.csv", organization_id),
                organization_reward_dashboard_csv(&response),
            )
        })
        .map_err(|error| {
            organization_reward_dashboard_error(error, ReportOperation::Export, organization_id)
        })
}

fn date_range(query: &HashMap<String, String>) -> (Option<NaiveDate>, Option<NaiveDate>) {
    (
        parse_date_query(query, "from"),
        parse_date_query(query, "to"),
    )
}

fn parse_date_query(query: &HashMap<String, String>, key: &str) -> Option<NaiveDate> {
    query
        .get(key)
        .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
}
