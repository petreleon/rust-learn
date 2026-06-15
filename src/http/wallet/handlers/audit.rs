use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::wallet::audit_wallet::{
    WalletAuditError, WalletAuditSubject, WalletAuditUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::WalletAuditResponse;

pub async fn get_my_wallet_audit(
    requester: AuthUser,
    audit: web::Data<Arc<dyn WalletAuditUseCase>>,
) -> impl Responder {
    wallet_audit_response(audit, requester.user_id(), WalletAuditSubject::OwnUser).await
}

pub async fn get_user_wallet_audit(
    requester: AuthUser,
    path: web::Path<i32>,
    audit: web::Data<Arc<dyn WalletAuditUseCase>>,
) -> impl Responder {
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
) -> impl Responder {
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
) -> HttpResponse {
    match audit.audit_wallet(actor_user_id, subject).await {
        Ok(audit) => HttpResponse::Ok().json(WalletAuditResponse::from(audit)),
        Err(error) => wallet_audit_error_response(error),
    }
}

fn wallet_audit_error_response(error: WalletAuditError) -> HttpResponse {
    match error {
        WalletAuditError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        WalletAuditError::UserNotFound => HttpResponse::NotFound().body("User not found"),
        WalletAuditError::OrganizationNotFound => {
            HttpResponse::NotFound().body("Organization not found")
        }
        WalletAuditError::WalletNotLinked => HttpResponse::NotFound().body("Wallet not linked"),
        WalletAuditError::UserPermissionDenied => {
            HttpResponse::Forbidden().body("User does not have wallet access")
        }
        WalletAuditError::OrganizationPermissionDenied => {
            HttpResponse::Forbidden().body("User does not have organization wallet access")
        }
        WalletAuditError::UserLoad(message) => {
            log::error!("event=wallet_audit_user_load_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load user")
        }
        WalletAuditError::OrganizationLoad(message) => {
            log::error!(
                "event=wallet_audit_organization_load_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to load organization")
        }
        WalletAuditError::UserAccessCheck(message) => {
            log::error!("event=wallet_audit_user_access_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to check wallet access")
        }
        WalletAuditError::OrganizationAccessCheck(message) => {
            log::error!(
                "event=wallet_audit_organization_access_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to check organization wallet access")
        }
        WalletAuditError::WalletLoad(message) => {
            log::error!("event=wallet_audit_wallet_load_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load wallet")
        }
        WalletAuditError::AuditLoad(message) | WalletAuditError::Database(message) => {
            log::error!("event=wallet_audit_load_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load wallet audit")
        }
    }
}
