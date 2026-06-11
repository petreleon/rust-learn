use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::db::schema::{organizations, users};
use crate::models::wallet::Wallet;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::services::wallet_audit_service;
use crate::services::wallet_service::{
    self, LinkedWallet, SetWalletTokenTaxRequest, WalletTokenOperation, WalletTokenTransferError,
    WalletTokenTransferRequest,
};
use crate::utils::request_auth::authenticated_user;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

#[derive(Serialize)]
struct WalletResponse {
    id: i32,
    owner_type: &'static str,
    user_id: Option<i32>,
    organization_id: Option<i32>,
    value: String,
}

#[derive(Serialize)]
struct WalletLinkResponse {
    wallet: WalletResponse,
    created: bool,
}

impl From<&Wallet> for WalletResponse {
    fn from(wallet: &Wallet) -> Self {
        let owner_type = if wallet.user_id.is_some() {
            "user"
        } else {
            "organization"
        };

        WalletResponse {
            id: wallet.id,
            owner_type,
            user_id: wallet.user_id,
            organization_id: wallet.organization_id,
            value: wallet.value.to_string(),
        }
    }
}

impl From<LinkedWallet> for WalletLinkResponse {
    fn from(linked_wallet: LinkedWallet) -> Self {
        WalletLinkResponse {
            wallet: WalletResponse::from(&linked_wallet.wallet),
            created: linked_wallet.created,
        }
    }
}

#[derive(Clone, Copy)]
enum WalletOperation {
    View,
    Link,
}

async fn ensure_user_exists(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<(), HttpResponse> {
    match users::table
        .find(user_id)
        .select(users::id)
        .first::<i32>(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => {
            Err(HttpResponse::NotFound().body("User not found"))
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to load user")),
    }
}

async fn ensure_organization_exists(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<(), HttpResponse> {
    match organizations::table
        .find(organization_id)
        .select(organizations::id)
        .first::<i32>(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => {
            Err(HttpResponse::NotFound().body("Organization not found"))
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to load organization")),
    }
}

async fn has_any_platform_permission(
    conn: &mut AsyncPgConnection,
    requester_id: i32,
    permissions: &[String],
) -> diesel::QueryResult<bool> {
    for permission in permissions {
        if user_permission_platform_request(conn, requester_id, permission).await? {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn has_any_organization_permission(
    conn: &mut AsyncPgConnection,
    requester_id: i32,
    organization_id: i32,
    permissions: &[String],
) -> diesel::QueryResult<bool> {
    for permission in permissions {
        if user_permission_organization_request(conn, requester_id, organization_id, permission)
            .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}
