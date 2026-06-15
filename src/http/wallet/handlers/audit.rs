use std::sync::Arc;

use actix_web::web;

use crate::application::wallet::audit_wallet::{WalletAuditSubject, WalletAuditUseCase};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::WalletAuditResponse;
use crate::http::wallet::errors::wallet_audit_error;

pub async fn get_my_wallet_audit(
    requester: AuthUser,
    audit: web::Data<Arc<dyn WalletAuditUseCase>>,
) -> Result<web::Json<WalletAuditResponse>, ApiError> {
    wallet_audit_response(audit, requester.user_id(), WalletAuditSubject::OwnUser).await
}

pub async fn get_user_wallet_audit(
    requester: AuthUser,
    path: web::Path<i32>,
    audit: web::Data<Arc<dyn WalletAuditUseCase>>,
) -> Result<web::Json<WalletAuditResponse>, ApiError> {
    wallet_audit_response(
        audit,
        requester.user_id(),
        WalletAuditSubject::User(path.into_inner()),
    )
    .await
}

pub async fn get_organization_wallet_audit(
    requester: AuthUser,
    path: web::Path<i32>,
    audit: web::Data<Arc<dyn WalletAuditUseCase>>,
) -> Result<web::Json<WalletAuditResponse>, ApiError> {
    wallet_audit_response(
        audit,
        requester.user_id(),
        WalletAuditSubject::Organization(path.into_inner()),
    )
    .await
}

async fn wallet_audit_response(
    audit: web::Data<Arc<dyn WalletAuditUseCase>>,
    actor_user_id: i32,
    subject: WalletAuditSubject,
) -> Result<web::Json<WalletAuditResponse>, ApiError> {
    audit
        .audit_wallet(actor_user_id, subject)
        .await
        .map(WalletAuditResponse::from)
        .map(web::Json)
        .map_err(wallet_audit_error)
}
