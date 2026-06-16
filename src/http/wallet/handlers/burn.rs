use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::wallet::burn_tokens::{TokenBurnSubject, TokenBurnUseCase};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::{
    OrganizationTokenBurnPermissionsResponse, TokenBurnLeaderboardQueryDto,
    TokenBurnLeaderboardResponse, TokenBurnRequestDto, TokenBurnResponse,
};
use crate::http::wallet::errors::token_burn_error;

pub async fn request_my_token_burn(
    requester: AuthUser,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
    body: web::Json<TokenBurnRequestDto>,
) -> Result<(web::Json<TokenBurnResponse>, StatusCode), ApiError> {
    request_token_burn_response(
        burns,
        requester.user_id(),
        TokenBurnSubject::OwnUser,
        body.into_inner(),
    )
    .await
}

pub async fn list_my_token_burns(
    requester: AuthUser,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
) -> Result<web::Json<Vec<TokenBurnResponse>>, ApiError> {
    list_token_burn_response(burns, requester.user_id(), TokenBurnSubject::OwnUser).await
}

pub async fn request_organization_token_burn(
    requester: AuthUser,
    path: web::Path<i32>,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
    body: web::Json<TokenBurnRequestDto>,
) -> Result<(web::Json<TokenBurnResponse>, StatusCode), ApiError> {
    request_token_burn_response(
        burns,
        requester.user_id(),
        TokenBurnSubject::Organization(path.into_inner()),
        body.into_inner(),
    )
    .await
}

pub async fn list_organization_token_burns(
    requester: AuthUser,
    path: web::Path<i32>,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
) -> Result<web::Json<Vec<TokenBurnResponse>>, ApiError> {
    list_token_burn_response(
        burns,
        requester.user_id(),
        TokenBurnSubject::Organization(path.into_inner()),
    )
    .await
}

pub async fn get_organization_token_burn_permissions(
    requester: AuthUser,
    path: web::Path<i32>,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
) -> Result<web::Json<OrganizationTokenBurnPermissionsResponse>, ApiError> {
    burns
        .load_organization_token_burn_permissions(requester.user_id(), path.into_inner())
        .await
        .map(OrganizationTokenBurnPermissionsResponse::from)
        .map(web::Json)
        .map_err(token_burn_error)
}

pub async fn get_token_burn_leaderboard(
    requester: AuthUser,
    query: web::Query<TokenBurnLeaderboardQueryDto>,
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
) -> Result<web::Json<TokenBurnLeaderboardResponse>, ApiError> {
    burns
        .load_token_burn_leaderboard(requester.user_id(), query.into_inner().into())
        .await
        .map(TokenBurnLeaderboardResponse::from)
        .map(web::Json)
        .map_err(token_burn_error)
}

async fn request_token_burn_response(
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
    actor_user_id: i32,
    subject: TokenBurnSubject,
    body: TokenBurnRequestDto,
) -> Result<(web::Json<TokenBurnResponse>, StatusCode), ApiError> {
    burns
        .request_token_burn(actor_user_id, subject, body.into())
        .await
        .map(TokenBurnResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(token_burn_error)
}

async fn list_token_burn_response(
    burns: web::Data<Arc<dyn TokenBurnUseCase>>,
    actor_user_id: i32,
    subject: TokenBurnSubject,
) -> Result<web::Json<Vec<TokenBurnResponse>>, ApiError> {
    burns
        .list_token_burns(actor_user_id, subject)
        .await
        .map(|rows| rows.into_iter().map(TokenBurnResponse::from).collect())
        .map(web::Json)
        .map_err(token_burn_error)
}
