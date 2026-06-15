use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::wallet::create_deposit_intent::{
    WalletDepositIntentError, WalletDepositIntentUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::{WalletDepositIntentRequestDto, WalletDepositIntentResponse};

pub async fn deposit_my_tokens(
    requester: AuthUser,
    deposit: web::Data<Arc<dyn WalletDepositIntentUseCase>>,
    body: web::Json<WalletDepositIntentRequestDto>,
) -> impl Responder {
    match deposit
        .create_deposit_intent(requester.user_id(), body.into_inner().into())
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
