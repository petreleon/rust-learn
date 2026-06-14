use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::reporting::organization_summary::{
    OrganizationSummaryError, OrganizationSummaryUseCase,
};
use crate::http::reporting::dto::{organization_summary_csv, OrganizationSummaryResponse};

pub async fn get_organization_summary(
    path: web::Path<i32>,
    summaries: web::Data<Arc<dyn OrganizationSummaryUseCase>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    match summaries.load_organization_summary(organization_id).await {
        Ok(output) => HttpResponse::Ok().json(OrganizationSummaryResponse::from(output)),
        Err(error) => organization_summary_error_response(error, "load", organization_id),
    }
}

pub async fn export_organization_summary(
    path: web::Path<i32>,
    summaries: web::Data<Arc<dyn OrganizationSummaryUseCase>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    match summaries.load_organization_summary(organization_id).await {
        Ok(output) => {
            let response = OrganizationSummaryResponse::from(output);
            csv_response(
                &format!("organization-{}-summary.csv", organization_id),
                organization_summary_csv(&response),
            )
        }
        Err(error) => organization_summary_error_response(error, "export", organization_id),
    }
}

fn organization_summary_error_response(
    error: OrganizationSummaryError,
    operation: &'static str,
    organization_id: i32,
) -> HttpResponse {
    match error {
        OrganizationSummaryError::NotFound => {
            HttpResponse::NotFound().body("Organization not found")
        }
        OrganizationSummaryError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        OrganizationSummaryError::Database(message) => {
            log::error!(
                "event=report_{}_failed scope=organization organization_id={} error={}",
                operation,
                organization_id,
                message
            );
            let label = if operation == "export" {
                "export organization report"
            } else {
                "load organization report"
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
