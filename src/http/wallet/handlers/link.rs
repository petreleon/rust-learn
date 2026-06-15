use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::wallet::link_wallet::{WalletLinkSubject, WalletLinkUseCase};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::WalletLinkResponse;
use crate::http::wallet::errors::wallet_link_error;

pub async fn link_my_wallet(
    requester: AuthUser,
    link: web::Data<Arc<dyn WalletLinkUseCase>>,
) -> Result<(web::Json<WalletLinkResponse>, StatusCode), ApiError> {
    wallet_link_response(link, requester.user_id(), WalletLinkSubject::OwnUser).await
}

pub async fn link_user_wallet(
    requester: AuthUser,
    path: web::Path<i32>,
    link: web::Data<Arc<dyn WalletLinkUseCase>>,
) -> Result<(web::Json<WalletLinkResponse>, StatusCode), ApiError> {
    wallet_link_response(
        link,
        requester.user_id(),
        WalletLinkSubject::User(path.into_inner()),
    )
    .await
}

pub async fn link_organization_wallet(
    requester: AuthUser,
    path: web::Path<i32>,
    link: web::Data<Arc<dyn WalletLinkUseCase>>,
) -> Result<(web::Json<WalletLinkResponse>, StatusCode), ApiError> {
    wallet_link_response(
        link,
        requester.user_id(),
        WalletLinkSubject::Organization(path.into_inner()),
    )
    .await
}

async fn wallet_link_response(
    link: web::Data<Arc<dyn WalletLinkUseCase>>,
    actor_user_id: i32,
    subject: WalletLinkSubject,
) -> Result<(web::Json<WalletLinkResponse>, StatusCode), ApiError> {
    link.link_wallet(actor_user_id, subject)
        .await
        .map(WalletLinkResponse::from)
        .map(wallet_link_success_response)
        .map_err(wallet_link_error)
}

fn wallet_link_success_response(
    response: WalletLinkResponse,
) -> (web::Json<WalletLinkResponse>, StatusCode) {
    let status = if response.created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    (web::Json(response), status)
}
