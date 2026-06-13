use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxError, WalletTokenTaxOperation, WalletTokenTaxUseCase,
};
use crate::http::wallet::dto::{
    SetWalletTokenTaxRequest, WalletTokenTaxResponse, WalletTokenTaxSettingsResponse,
};
use crate::utils::request_auth::authenticated_user;

pub async fn list_wallet_token_taxes(
    req: HttpRequest,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
) -> impl Responder {
    if let Err(response) = authenticated_user(&req) {
        return response;
    }

    match tax.list_token_taxes().await {
        Ok(settings) => HttpResponse::Ok().json(WalletTokenTaxSettingsResponse::from(settings)),
        Err(error) => wallet_token_tax_error_response(error),
    }
}

pub async fn set_deposit_tax(
    req: HttpRequest,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
    body: web::Json<SetWalletTokenTaxRequest>,
) -> impl Responder {
    set_wallet_token_tax(
        req,
        tax,
        WalletTokenTaxOperation::Deposit,
        body.into_inner(),
    )
    .await
}

pub async fn set_retire_tax(
    req: HttpRequest,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
    body: web::Json<SetWalletTokenTaxRequest>,
) -> impl Responder {
    set_wallet_token_tax(req, tax, WalletTokenTaxOperation::Retire, body.into_inner()).await
}

async fn set_wallet_token_tax(
    req: HttpRequest,
    tax: web::Data<Arc<dyn WalletTokenTaxUseCase>>,
    operation: WalletTokenTaxOperation,
    body: SetWalletTokenTaxRequest,
) -> HttpResponse {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match tax
        .set_token_tax(requester.user_id, operation, body.tax_amount)
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
