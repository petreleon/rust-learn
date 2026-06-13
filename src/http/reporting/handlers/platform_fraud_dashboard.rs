use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::reporting::platform_fraud_dashboard::{
    PlatformFraudDashboardError, PlatformFraudDashboardUseCase,
};
use crate::http::reporting::dto::{platform_fraud_dashboard_csv, PlatformFraudDashboardResponse};

pub async fn get_platform_fraud_dashboard(
    dashboard: web::Data<Arc<dyn PlatformFraudDashboardUseCase>>,
) -> impl Responder {
    match dashboard.load_platform_fraud_dashboard().await {
        Ok(output) => HttpResponse::Ok().json(PlatformFraudDashboardResponse::from(output)),
        Err(error) => platform_fraud_dashboard_error_response(error, "load"),
    }
}

pub async fn export_platform_fraud_dashboard(
    dashboard: web::Data<Arc<dyn PlatformFraudDashboardUseCase>>,
) -> impl Responder {
    match dashboard.load_platform_fraud_dashboard().await {
        Ok(output) => {
            let response = PlatformFraudDashboardResponse::from(output);
            csv_response(
                "platform-fraud-dashboard.csv",
                platform_fraud_dashboard_csv(&response),
            )
        }
        Err(error) => platform_fraud_dashboard_error_response(error, "export"),
    }
}

fn platform_fraud_dashboard_error_response(
    error: PlatformFraudDashboardError,
    operation: &'static str,
) -> HttpResponse {
    match error {
        PlatformFraudDashboardError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        PlatformFraudDashboardError::Database(message) => {
            log::error!(
                "event=report_{}_failed scope=platform report=fraud_dashboard error={}",
                operation,
                message
            );
            let label = if operation == "export" {
                "export fraud dashboard"
            } else {
                "load fraud dashboard"
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
