use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::reporting::platform_csv_exports::{
    PlatformCsvExportError, PlatformCsvExportsUseCase,
};
use crate::http::reporting::dto::{
    platform_delegated_permissions_csv, platform_reward_approvals_csv,
    platform_teacher_applications_csv, platform_token_payouts_csv, platform_wallet_credits_csv,
};

pub async fn export_platform_teacher_applications(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> impl Responder {
    match exports.load_platform_teacher_application_exports().await {
        Ok(rows) => csv_response(
            "platform-teacher-applications.csv",
            platform_teacher_applications_csv(&rows),
        ),
        Err(error) => export_error_response(error, "teacher_applications", "teacher applications"),
    }
}

pub async fn export_platform_reward_approvals(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> impl Responder {
    match exports.load_platform_reward_approval_exports().await {
        Ok(rows) => csv_response(
            "platform-reward-approvals.csv",
            platform_reward_approvals_csv(&rows),
        ),
        Err(error) => export_error_response(error, "reward_approvals", "reward approvals"),
    }
}

pub async fn export_platform_token_payouts(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> impl Responder {
    match exports.load_platform_token_payout_exports().await {
        Ok(rows) => csv_response(
            "platform-token-payouts.csv",
            platform_token_payouts_csv(&rows),
        ),
        Err(error) => export_error_response(error, "token_payouts", "token payouts"),
    }
}

pub async fn export_platform_wallet_credits(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> impl Responder {
    match exports.load_platform_wallet_credit_exports().await {
        Ok(rows) => csv_response(
            "platform-wallet-credits.csv",
            platform_wallet_credits_csv(&rows),
        ),
        Err(error) => export_error_response(error, "wallet_credits", "wallet credits"),
    }
}

pub async fn export_platform_delegated_permissions(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> impl Responder {
    match exports.load_platform_delegated_permission_exports().await {
        Ok(rows) => csv_response(
            "platform-delegated-permissions.csv",
            platform_delegated_permissions_csv(&rows),
        ),
        Err(error) => {
            export_error_response(error, "delegated_permissions", "delegated permissions")
        }
    }
}

fn export_error_response(
    error: PlatformCsvExportError,
    report: &'static str,
    label: &'static str,
) -> HttpResponse {
    match error {
        PlatformCsvExportError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        PlatformCsvExportError::Database(message) => {
            log::error!(
                "event=report_export_failed scope=platform report={} error={}",
                report,
                message
            );
            HttpResponse::InternalServerError().body(format!("Failed to export {}", label))
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
