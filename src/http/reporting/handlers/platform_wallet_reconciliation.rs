use std::sync::Arc;

use actix_web::web;

use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationUseCase;
use crate::http::errors::ApiError;
use crate::http::reporting::dto::PlatformWalletReconciliationResponse;
use crate::http::reporting::errors::platform_wallet_reconciliation_error;

pub async fn get_platform_wallet_reconciliation(
    reconciliation: web::Data<Arc<dyn PlatformWalletReconciliationUseCase>>,
) -> Result<web::Json<PlatformWalletReconciliationResponse>, ApiError> {
    reconciliation
        .load_platform_wallet_reconciliation()
        .await
        .map(PlatformWalletReconciliationResponse::from)
        .map(web::Json)
        .map_err(platform_wallet_reconciliation_error)
}
