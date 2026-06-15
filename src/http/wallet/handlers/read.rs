use std::sync::Arc;

use actix_web::web;

use crate::application::wallet::read_wallet::{WalletReadSubject, WalletReadUseCase};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::WalletResponse;
use crate::http::wallet::errors::wallet_read_error;

pub async fn get_my_wallet(
    requester: AuthUser,
    read: web::Data<Arc<dyn WalletReadUseCase>>,
) -> Result<web::Json<WalletResponse>, ApiError> {
    wallet_read_response(read, requester.user_id(), WalletReadSubject::OwnUser).await
}

pub async fn get_user_wallet(
    requester: AuthUser,
    path: web::Path<i32>,
    read: web::Data<Arc<dyn WalletReadUseCase>>,
) -> Result<web::Json<WalletResponse>, ApiError> {
    wallet_read_response(
        read,
        requester.user_id(),
        WalletReadSubject::User(path.into_inner()),
    )
    .await
}

pub async fn get_organization_wallet(
    requester: AuthUser,
    path: web::Path<i32>,
    read: web::Data<Arc<dyn WalletReadUseCase>>,
) -> Result<web::Json<WalletResponse>, ApiError> {
    wallet_read_response(
        read,
        requester.user_id(),
        WalletReadSubject::Organization(path.into_inner()),
    )
    .await
}

async fn wallet_read_response(
    read: web::Data<Arc<dyn WalletReadUseCase>>,
    actor_user_id: i32,
    subject: WalletReadSubject,
) -> Result<web::Json<WalletResponse>, ApiError> {
    read.read_wallet(actor_user_id, subject)
        .await
        .map(WalletResponse::from)
        .map(web::Json)
        .map_err(wallet_read_error)
}
