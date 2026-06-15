use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::wallet::link_wallet::{
    WalletLinkError, WalletLinkSubject, WalletLinkUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::wallet::dto::WalletLinkResponse;

pub async fn link_my_wallet(
    requester: AuthUser,
    link: web::Data<Arc<dyn WalletLinkUseCase>>,
) -> impl Responder {
    wallet_link_response(link, requester.user_id(), WalletLinkSubject::OwnUser).await
}

pub async fn link_user_wallet(
    requester: AuthUser,
    path: web::Path<i32>,
    link: web::Data<Arc<dyn WalletLinkUseCase>>,
) -> impl Responder {
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
) -> impl Responder {
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
) -> HttpResponse {
    match link.link_wallet(actor_user_id, subject).await {
        Ok(linked_wallet) => wallet_link_success_response(linked_wallet.into()),
        Err(error) => wallet_link_error_response(error),
    }
}

fn wallet_link_success_response(response: WalletLinkResponse) -> HttpResponse {
    if response.created {
        HttpResponse::Created().json(response)
    } else {
        HttpResponse::Ok().json(response)
    }
}

fn wallet_link_error_response(error: WalletLinkError) -> HttpResponse {
    match error {
        WalletLinkError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        WalletLinkError::UserNotFound => HttpResponse::NotFound().body("User not found"),
        WalletLinkError::OrganizationNotFound => {
            HttpResponse::NotFound().body("Organization not found")
        }
        WalletLinkError::KycRequired => {
            HttpResponse::Conflict().body("KYC verification is required before wallet actions")
        }
        WalletLinkError::UserPermissionDenied => {
            HttpResponse::Forbidden().body("User does not have wallet access")
        }
        WalletLinkError::OrganizationPermissionDenied => {
            HttpResponse::Forbidden().body("User does not have organization wallet access")
        }
        WalletLinkError::UserLoad(message) => {
            log::error!("event=wallet_link_user_load_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load user")
        }
        WalletLinkError::OrganizationLoad(message) => {
            log::error!(
                "event=wallet_link_organization_load_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to load organization")
        }
        WalletLinkError::UserAccessCheck(message) => {
            log::error!("event=wallet_link_user_access_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to check wallet access")
        }
        WalletLinkError::OrganizationAccessCheck(message) => {
            log::error!(
                "event=wallet_link_organization_access_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to check organization wallet access")
        }
        WalletLinkError::KycLoad(message) => {
            log::error!("event=wallet_link_kyc_load_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load user KYC status")
        }
        WalletLinkError::WalletLoad(message) | WalletLinkError::WalletCreate(message) => {
            log::error!("event=wallet_link_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to link wallet")
        }
    }
}
