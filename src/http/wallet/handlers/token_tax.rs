use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxError, WalletTokenTaxOperation, WalletTokenTaxUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::{
    SetWalletTokenTaxRequest, WalletTokenTaxResponse, WalletTokenTaxSettingsResponse,
};

pub async fn list_wallet_token_taxes(
    _requester: AuthUser,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
) -> impl Responder {
    match tax.list_token_taxes().await {
        Ok(settings) => HttpResponse::Ok().json(WalletTokenTaxSettingsResponse::from(settings)),
        Err(error) => wallet_token_tax_error_response(error),
    }
}

pub async fn set_deposit_tax(
    requester: AuthUser,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
    body: web::Json<SetWalletTokenTaxRequest>,
) -> impl Responder {
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
) -> impl Responder {
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
) -> HttpResponse {
    match tax
        .set_token_tax(actor_user_id, operation, body.tax_amount)
        .await
    {
        Ok(view) => HttpResponse::Ok().json(WalletTokenTaxResponse::from(view)),
        Err(error) => wallet_token_tax_error_response(error),
    }
}

fn wallet_token_tax_error_response(error: WalletTokenTaxError) -> HttpResponse {
    match error {
        WalletTokenTaxError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        WalletTokenTaxError::PermissionDenied => {
            HttpResponse::Forbidden().body("User does not have wallet tax permission")
        }
        WalletTokenTaxError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        WalletTokenTaxError::PermissionCheck(message)
        | WalletTokenTaxError::TaxLoad(message)
        | WalletTokenTaxError::TaxStore(message) => {
            log::error!("event=wallet_token_tax_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process wallet token transfer")
        }
    }
}
