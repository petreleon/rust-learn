use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::wallet::read_wallet::{
    WalletReadError, WalletReadSubject, WalletReadUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::WalletResponse;

pub async fn get_my_wallet(
    requester: AuthUser,
    read: web::Data<Arc<dyn WalletReadUseCase>>,
) -> impl Responder {
    wallet_read_response(read, requester.user_id(), WalletReadSubject::OwnUser).await
}

pub async fn get_user_wallet(
    requester: AuthUser,
    path: web::Path<i32>,
    read: web::Data<Arc<dyn WalletReadUseCase>>,
) -> impl Responder {
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
) -> impl Responder {
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
) -> HttpResponse {
    match read.read_wallet(actor_user_id, subject).await {
        Ok(wallet) => HttpResponse::Ok().json(WalletResponse::from(wallet)),
        Err(error) => wallet_read_error_response(error),
    }
}

fn wallet_read_error_response(error: WalletReadError) -> HttpResponse {
    match error {
        WalletReadError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        WalletReadError::UserNotFound => HttpResponse::NotFound().body("User not found"),
        WalletReadError::OrganizationNotFound => {
            HttpResponse::NotFound().body("Organization not found")
        }
        WalletReadError::WalletNotLinked => HttpResponse::NotFound().body("Wallet not linked"),
        WalletReadError::UserPermissionDenied => {
            HttpResponse::Forbidden().body("User does not have wallet access")
        }
        WalletReadError::OrganizationPermissionDenied => {
            HttpResponse::Forbidden().body("User does not have organization wallet access")
        }
        WalletReadError::UserLoad(message) => {
            log::error!("event=wallet_read_user_load_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load user")
        }
        WalletReadError::OrganizationLoad(message) => {
            log::error!(
                "event=wallet_read_organization_load_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to load organization")
        }
        WalletReadError::UserAccessCheck(message) => {
            log::error!("event=wallet_read_user_access_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to check wallet access")
        }
        WalletReadError::OrganizationAccessCheck(message) => {
            log::error!(
                "event=wallet_read_organization_access_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to check organization wallet access")
        }
        WalletReadError::WalletLoad(message) => {
            log::error!("event=wallet_read_wallet_load_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load wallet")
        }
    }
}
