use std::sync::Arc;

use actix_web::web;

use crate::application::wallet::burn_tokens::TokenBurnUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::{TokenBurnReconciliationRequestDto, TokenBurnResponse};
use crate::http::wallet::errors::token_burn_error;

pub async fn list_token_burn_reconciliation_queue(
    requester: AuthUser,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
) -> Result<web::Json<Vec<TokenBurnResponse>>, ApiError> {
    burns
        .list_token_burn_reconciliation_queue(requester.user_id())
        .await
        .map(token_burn_rows)
        .map(web::Json)
        .map_err(token_burn_error)
}

pub async fn list_failed_token_burns(
    requester: AuthUser,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
) -> Result<web::Json<Vec<TokenBurnResponse>>, ApiError> {
    burns
        .list_failed_token_burns(requester.user_id())
        .await
        .map(token_burn_rows)
        .map(web::Json)
        .map_err(token_burn_error)
}

pub async fn reconcile_token_burn(
    requester: AuthUser,
    path: web::Path<i64>,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
    body: web::Json<TokenBurnReconciliationRequestDto>,
) -> Result<web::Json<TokenBurnResponse>, ApiError> {
    burns
        .reconcile_token_burn(
            requester.user_id(),
            path.into_inner(),
            body.into_inner().into(),
        )
        .await
        .map(TokenBurnResponse::from)
        .map(web::Json)
        .map_err(token_burn_error)
}

fn token_burn_rows(
    rows: Vec<crate::application::wallet::burn_tokens::TokenBurnView>,
) -> Vec<TokenBurnResponse> {
    rows.into_iter().map(TokenBurnResponse::from).collect()
}
