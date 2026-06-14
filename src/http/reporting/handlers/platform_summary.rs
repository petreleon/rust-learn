use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::reporting::platform_summary::{
    PlatformSummaryError, PlatformSummaryUseCase,
};
use crate::http::reporting::dto::{platform_summary_csv, PlatformSummaryResponse};

pub async fn get_platform_summary(
    summary: web::Data<Arc<dyn PlatformSummaryUseCase>>,
) -> impl Responder {
    match summary.load_platform_summary().await {
        Ok(output) => HttpResponse::Ok().json(PlatformSummaryResponse::from(output)),
        Err(error) => platform_summary_error_response(error, "load"),
    }
}

pub async fn export_platform_summary(
    summary: web::Data<Arc<dyn PlatformSummaryUseCase>>,
) -> impl Responder {
    match summary.load_platform_summary().await {
        Ok(output) => {
            let response = PlatformSummaryResponse::from(output);
            csv_response("platform-summary.csv", platform_summary_csv(&response))
        }
        Err(error) => platform_summary_error_response(error, "export"),
    }
}

fn platform_summary_error_response(
    error: PlatformSummaryError,
    operation: &'static str,
) -> HttpResponse {
    match error {
        PlatformSummaryError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        PlatformSummaryError::Database(message) => {
            log::error!(
                "event=report_{}_failed scope=platform error={}",
                operation,
                message
            );
            let action = if operation == "export" {
                "export"
            } else {
                "load"
            };
            HttpResponse::InternalServerError()
                .body(format!("Failed to {} platform report", action))
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
