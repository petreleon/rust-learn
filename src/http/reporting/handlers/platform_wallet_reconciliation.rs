use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::reporting::platform_wallet_reconciliation::{
    PlatformWalletReconciliationError, PlatformWalletReconciliationUseCase,
};
use crate::http::reporting::dto::PlatformWalletReconciliationResponse;

pub async fn get_platform_wallet_reconciliation(
    reconciliation: web::Data<Arc<dyn PlatformWalletReconciliationUseCase>>,
) -> impl Responder {
    match reconciliation.load_platform_wallet_reconciliation().await {
        Ok(output) => HttpResponse::Ok().json(PlatformWalletReconciliationResponse::from(output)),
        Err(error) => platform_wallet_reconciliation_error_response(error),
    }
}

fn platform_wallet_reconciliation_error_response(
    error: PlatformWalletReconciliationError,
) -> HttpResponse {
    match error {
        PlatformWalletReconciliationError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        PlatformWalletReconciliationError::Database(message) => {
            log::error!(
                "event=report_load_failed scope=platform report=wallet_reconciliation error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to load wallet reconciliation")
        }
    }
}
