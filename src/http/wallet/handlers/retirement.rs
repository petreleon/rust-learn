use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::wallet::retire_tokens::{WalletRetirementError, WalletRetirementUseCase};
use crate::http::extractors::request_auth::authenticated_user;
use crate::http::wallet::dto::{WalletRetirementRequestDto, WalletRetirementResponse};

pub async fn retire_my_tokens(
    req: HttpRequest,
    retirement: web::Data<Arc<dyn WalletRetirementUseCase>>,
    body: web::Json<WalletRetirementRequestDto>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match retirement
        .retire_tokens(requester.user_id, body.into_inner().into())
        .await
    {
        Ok(result) => HttpResponse::Created().json(WalletRetirementResponse::from(result)),
        Err(error) => wallet_retirement_error_response(error),
    }
}

fn wallet_retirement_error_response(error: WalletRetirementError) -> HttpResponse {
    match error {
        WalletRetirementError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        WalletRetirementError::KycRequired => {
            HttpResponse::Conflict().body("KYC verification is required before wallet actions")
        }
        WalletRetirementError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        WalletRetirementError::InsufficientFunds => {
            HttpResponse::Conflict().body("Insufficient wallet balance")
        }
        WalletRetirementError::KycLoad(message)
        | WalletRetirementError::TaxLoad(message)
        | WalletRetirementError::WalletLoad(message)
        | WalletRetirementError::WalletCreate(message)
        | WalletRetirementError::RetirementCreate(message) => {
            log::error!("event=wallet_retirement_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process wallet token transfer")
        }
    }
}
