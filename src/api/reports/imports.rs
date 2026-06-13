use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::services::reporting_service::{
    organization_reward_dashboard, organization_reward_dashboard_csv,
    platform_delegated_permissions_csv, platform_reward_approvals_csv, platform_reward_dashboard,
    platform_reward_dashboard_csv, platform_teacher_applications_csv, platform_token_payouts_csv,
    platform_wallet_credits_csv, platform_wallet_reconciliation,
};
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;

fn parse_date_query(query: &HashMap<String, String>, key: &str) -> Option<chrono::NaiveDate> {
    query
        .get(key)
        .and_then(|v| chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d").ok())
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

async fn get_platform_reward_dashboard(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_reward_dashboard(&mut conn).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(err) => {
            log::error!(
                "event=report_load_failed scope=platform report=reward_dashboard error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to load reward dashboard")
        }
    }
}

async fn export_platform_reward_dashboard(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_reward_dashboard(&mut conn).await {
        Ok(dashboard) => csv_response(
            "platform-reward-dashboard.csv",
            platform_reward_dashboard_csv(&dashboard),
        ),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=reward_dashboard error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export reward dashboard")
        }
    }
}

async fn export_platform_teacher_applications(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_teacher_applications_csv(&mut conn).await {
        Ok(csv) => csv_response("platform-teacher-applications.csv", csv),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=teacher_applications error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export teacher applications")
        }
    }
}
