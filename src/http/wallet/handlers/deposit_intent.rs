use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::wallet::create_deposit_intent::WalletDepositIntentUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::{WalletDepositIntentRequestDto, WalletDepositIntentResponse};
use crate::http::wallet::errors::wallet_deposit_intent_error;

pub async fn deposit_my_tokens(
    requester: AuthUser,
    deposit: web::Data<Arc<dyn WalletDepositIntentUseCase>>,
    body: web::Json<WalletDepositIntentRequestDto>,
) -> Result<(web::Json<WalletDepositIntentResponse>, StatusCode), ApiError> {
    deposit
        .create_deposit_intent(requester.user_id(), body.into_inner().into())
        .await
        .map(WalletDepositIntentResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(wallet_deposit_intent_error)
}
