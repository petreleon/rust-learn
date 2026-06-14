use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, PlatformRewardDashboardUseCase,
};
use crate::http::reporting::dto::{platform_reward_dashboard_csv, PlatformRewardDashboardResponse};

pub async fn get_platform_reward_dashboard(
    dashboard: web::Data<Arc<dyn PlatformRewardDashboardUseCase>>,
) -> impl Responder {
    match dashboard.load_platform_reward_dashboard().await {
        Ok(output) => HttpResponse::Ok().json(PlatformRewardDashboardResponse::from(output)),
        Err(error) => platform_reward_dashboard_error_response(error, "load"),
    }
}

pub async fn export_platform_reward_dashboard(
    dashboard: web::Data<Arc<dyn PlatformRewardDashboardUseCase>>,
) -> impl Responder {
    match dashboard.load_platform_reward_dashboard().await {
        Ok(output) => {
            let response = PlatformRewardDashboardResponse::from(output);
            csv_response(
                "platform-reward-dashboard.csv",
                platform_reward_dashboard_csv(&response),
            )
        }
        Err(error) => platform_reward_dashboard_error_response(error, "export"),
    }
}

fn platform_reward_dashboard_error_response(
    error: PlatformRewardDashboardError,
    operation: &'static str,
) -> HttpResponse {
    match error {
        PlatformRewardDashboardError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        PlatformRewardDashboardError::Database(message) => {
            log::error!(
                "event=report_{}_failed scope=platform report=reward_dashboard error={}",
                operation,
                message
            );
            let label = if operation == "export" {
                "export reward dashboard"
            } else {
                "load reward dashboard"
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
