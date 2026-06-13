use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::wallet::create_deposit_intent::{
    WalletDepositIntentError, WalletDepositIntentUseCase,
};
use crate::http::wallet::dto::{WalletDepositIntentRequestDto, WalletDepositIntentResponse};
use crate::utils::request_auth::authenticated_user;

pub async fn deposit_my_tokens(
    req: HttpRequest,
    deposit: web::Data<Arc<dyn WalletDepositIntentUseCase>>,
    body: web::Json<WalletDepositIntentRequestDto>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match deposit
        .create_deposit_intent(requester.user_id, body.into_inner().into())
        .await
    {
        Ok(intent) => HttpResponse::Created().json(WalletDepositIntentResponse::from(intent)),
        Err(error) => wallet_deposit_intent_error_response(error),
    }
}

fn wallet_deposit_intent_error_response(error: WalletDepositIntentError) -> HttpResponse {
    match error {
        WalletDepositIntentError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        WalletDepositIntentError::KycRequired => {
            HttpResponse::Conflict().body("KYC verification is required before wallet actions")
        }
        WalletDepositIntentError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        WalletDepositIntentError::KycLoad(message)
        | WalletDepositIntentError::TaxLoad(message)
        | WalletDepositIntentError::ConfigurationLoad(message)
        | WalletDepositIntentError::WalletLoad(message)
        | WalletDepositIntentError::WalletCreate(message)
        | WalletDepositIntentError::DepositIntentCreate(message) => {
            log::error!("event=wallet_deposit_intent_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process wallet token transfer")
        }
    }
}
