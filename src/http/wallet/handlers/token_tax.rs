use std::sync::Arc;

use actix_web::web;

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxOperation, WalletTokenTaxUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::{
    SetWalletTokenTaxRequest, WalletTokenTaxAuditEventResponse, WalletTokenTaxResponse,
    WalletTokenTaxSettingsResponse,
};
use crate::http::wallet::errors::wallet_token_tax_error;

pub async fn list_wallet_token_taxes(
    requester: AuthUser,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
) -> Result<web::Json<WalletTokenTaxSettingsResponse>, ApiError> {
    tax.list_token_taxes(requester.user_id())
        .await
        .map(WalletTokenTaxSettingsResponse::from)
        .map(web::Json)
        .map_err(wallet_token_tax_error)
}

pub async fn list_wallet_token_tax_audit(
    requester: AuthUser,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
) -> Result<web::Json<Vec<WalletTokenTaxAuditEventResponse>>, ApiError> {
    tax.list_token_tax_audit(requester.user_id())
        .await
        .map(|events| {
            events
                .into_iter()
                .map(WalletTokenTaxAuditEventResponse::from)
                .collect()
        })
        .map(web::Json)
        .map_err(wallet_token_tax_error)
}

pub async fn set_deposit_tax(
    requester: AuthUser,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
    body: web::Json<SetWalletTokenTaxRequest>,
) -> Result<web::Json<WalletTokenTaxResponse>, ApiError> {
    set_wallet_token_tax(
        requester.user_id(),
        tax,
        WalletTokenTaxOperation::Deposit,
        body.into_inner(),
    )
    .await
}

pub async fn set_retire_tax(
    requester: AuthUser,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
    body: web::Json<SetWalletTokenTaxRequest>,
) -> Result<web::Json<WalletTokenTaxResponse>, ApiError> {
    set_wallet_token_tax(
        requester.user_id(),
        tax,
        WalletTokenTaxOperation::Retire,
        body.into_inner(),
    )
    .await
}

async fn set_wallet_token_tax(
    actor_user_id: i32,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
    operation: WalletTokenTaxOperation,
    body: SetWalletTokenTaxRequest,
) -> Result<web::Json<WalletTokenTaxResponse>, ApiError> {
    tax.set_token_tax(actor_user_id, operation, body.tax_amount)
        .await
        .map(WalletTokenTaxResponse::from)
        .map(web::Json)
        .map_err(wallet_token_tax_error)
}
