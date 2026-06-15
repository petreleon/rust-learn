use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::wallet::retire_tokens::WalletRetirementUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::{WalletRetirementRequestDto, WalletRetirementResponse};
use crate::http::wallet::errors::wallet_retirement_error;

pub async fn retire_my_tokens(
    requester: AuthUser,
    retirement: web::Data<Arc<dyn WalletRetirementUseCase>>,
    body: web::Json<WalletRetirementRequestDto>,
) -> Result<(web::Json<WalletRetirementResponse>, StatusCode), ApiError> {
    retirement
        .retire_tokens(requester.user_id(), body.into_inner().into())
        .await
        .map(WalletRetirementResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(wallet_retirement_error)
}
