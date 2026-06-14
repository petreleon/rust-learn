use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;

use crate::application::reporting::organization_reward_dashboard::{
    OrganizationRewardDashboardError, OrganizationRewardDashboardUseCase,
};
use crate::http::reporting::dto::{
    organization_reward_dashboard_csv, OrganizationRewardDashboardResponse,
};

pub async fn get_organization_reward_dashboard(
    path: web::Path<i32>,
    dashboard: web::Data<Arc<dyn OrganizationRewardDashboardUseCase>>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let (from, to) = date_range(&query);
    match dashboard
        .load_organization_reward_dashboard(organization_id, from, to)
        .await
    {
        Ok(output) => HttpResponse::Ok().json(OrganizationRewardDashboardResponse::from(output)),
        Err(error) => organization_reward_dashboard_error_response(error, "load", organization_id),
    }
}

pub async fn export_organization_reward_dashboard(
    path: web::Path<i32>,
    dashboard: web::Data<Arc<dyn OrganizationRewardDashboardUseCase>>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let (from, to) = date_range(&query);
    match dashboard
        .load_organization_reward_dashboard(organization_id, from, to)
        .await
    {
        Ok(output) => {
            let response = OrganizationRewardDashboardResponse::from(output);
            csv_response(
                &format!("organization-{}-reward-dashboard.csv", organization_id),
                organization_reward_dashboard_csv(&response),
            )
        }
        Err(error) => {
            organization_reward_dashboard_error_response(error, "export", organization_id)
        }
    }
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

fn organization_reward_dashboard_error_response(
    error: OrganizationRewardDashboardError,
    operation: &'static str,
    organization_id: i32,
) -> HttpResponse {
    match error {
        OrganizationRewardDashboardError::NotFound => {
            HttpResponse::NotFound().body("Organization not found")
        }
        OrganizationRewardDashboardError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        OrganizationRewardDashboardError::Database(message) => {
            log::error!(
                "event=report_{}_failed scope=organization report=reward_dashboard organization_id={} error={}",
                operation,
                organization_id,
                message
            );
            let label = if operation == "export" {
                "export organization reward dashboard"
            } else {
                "load organization reward dashboard"
            };
            HttpResponse::InternalServerError().body(format!("Failed to {}", label))
        }
    }
}

fn csv_response(filename: &str, body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/csv; charset=utf-8")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(body)
}
