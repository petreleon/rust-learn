use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::db::schema::{organizations, users};
use crate::models::user_jwt::UserJWT;
use crate::models::wallet::Wallet;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::services::wallet_audit_service;
use crate::services::wallet_service::{self, LinkedWallet};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
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

fn current_user(req: &HttpRequest) -> Result<UserJWT, HttpResponse> {
    req.extensions()
        .get::<UserJWT>()
        .cloned()
        .ok_or_else(|| HttpResponse::Unauthorized().body("Unauthorized access"))
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

async fn ensure_user_wallet_access(
    conn: &mut AsyncPgConnection,
    requester_id: i32,
    target_user_id: i32,
    operation: WalletOperation,
) -> Result<(), HttpResponse> {
    if requester_id == target_user_id {
        return Ok(());
    }

    let permissions = match operation {
        WalletOperation::View => vec![
            Permissions::VIEW_WALLET.to_string(),
            Permissions::VIEW_TRANSACTIONS.to_string(),
            Permissions::RECONCILE_WALLETS.to_string(),
            Permissions::MANAGE_WALLETS.to_string(),
        ],
        WalletOperation::Link => vec![
            Permissions::CREATE_WALLET.to_string(),
            Permissions::MANAGE_WALLETS.to_string(),
        ],
    };

    match has_any_platform_permission(conn, requester_id, permissions.as_slice()).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(HttpResponse::Forbidden().body("User does not have wallet access")),
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to check wallet access")),
    }
}

async fn ensure_organization_wallet_access(
    conn: &mut AsyncPgConnection,
    requester_id: i32,
    organization_id: i32,
    operation: WalletOperation,
) -> Result<(), HttpResponse> {
    let platform_permissions = match operation {
        WalletOperation::View => vec![
            Permissions::VIEW_WALLET.to_string(),
            Permissions::VIEW_TRANSACTIONS.to_string(),
            Permissions::RECONCILE_WALLETS.to_string(),
            Permissions::MANAGE_WALLETS.to_string(),
        ],
        WalletOperation::Link => vec![
            Permissions::CREATE_WALLET.to_string(),
            Permissions::MANAGE_WALLETS.to_string(),
        ],
    };

    match has_any_platform_permission(conn, requester_id, platform_permissions.as_slice()).await {
        Ok(true) => return Ok(()),
        Ok(false) => {}
        Err(_) => {
            return Err(HttpResponse::InternalServerError().body("Failed to check wallet access"))
        }
    }

    let organization_permissions = match operation {
        WalletOperation::View => vec![
            Permissions::MANAGE_ORG_WALLETS.to_string(),
            Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
            Permissions::MANAGE_ORG_REWARD_BUDGET.to_string(),
        ],
        WalletOperation::Link => vec![Permissions::MANAGE_ORG_WALLETS.to_string()],
    };

    match has_any_organization_permission(
        conn,
        requester_id,
        organization_id,
        organization_permissions.as_slice(),
    )
    .await
    {
        Ok(true) => Ok(()),
        Ok(false) => {
            Err(HttpResponse::Forbidden().body("User does not have organization wallet access"))
        }
        Err(_) => {
            Err(HttpResponse::InternalServerError()
                .body("Failed to check organization wallet access"))
        }
    }
}

fn wallet_not_linked_response() -> HttpResponse {
    HttpResponse::NotFound().body("Wallet not linked")
}

fn link_response(linked_wallet: LinkedWallet) -> HttpResponse {
    let created = linked_wallet.created;
    let response = WalletLinkResponse::from(linked_wallet);

    if created {
        HttpResponse::Created().json(response)
    } else {
        HttpResponse::Ok().json(response)
    }
}

async fn get_my_wallet(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_user_wallet_by_id(pool, requester.user_id, requester.user_id).await
}

async fn get_my_wallet_audit(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_user_wallet_audit_by_id(pool, requester.user_id, requester.user_id).await
}

async fn link_my_wallet(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    link_user_wallet_by_id(pool, requester.user_id, requester.user_id).await
}

async fn get_user_wallet(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_user_wallet_by_id(pool, requester.user_id, path.into_inner()).await
}

async fn get_user_wallet_audit(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_user_wallet_audit_by_id(pool, requester.user_id, path.into_inner()).await
}

async fn link_user_wallet(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    link_user_wallet_by_id(pool, requester.user_id, path.into_inner()).await
}

async fn get_user_wallet_by_id(
    pool: web::Data<db::DbPool>,
    requester_id: i32,
    target_user_id: i32,
) -> HttpResponse {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    if let Err(response) = ensure_user_wallet_access(
        &mut conn,
        requester_id,
        target_user_id,
        WalletOperation::View,
    )
    .await
    {
        return response;
    }
    if let Err(response) = ensure_user_exists(&mut conn, target_user_id).await {
        return response;
    }

    match wallet_service::find_user_wallet(&mut conn, target_user_id).await {
        Ok(Some(wallet)) => HttpResponse::Ok().json(WalletResponse::from(&wallet)),
        Ok(None) => wallet_not_linked_response(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to load wallet"),
    }
}

async fn get_user_wallet_audit_by_id(
    pool: web::Data<db::DbPool>,
    requester_id: i32,
    target_user_id: i32,
) -> HttpResponse {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    if let Err(response) = ensure_user_wallet_access(
        &mut conn,
        requester_id,
        target_user_id,
        WalletOperation::View,
    )
    .await
    {
        return response;
    }
    if let Err(response) = ensure_user_exists(&mut conn, target_user_id).await {
        return response;
    }

    match wallet_service::find_user_wallet(&mut conn, target_user_id).await {
        Ok(Some(wallet)) => match wallet_audit_service::build_wallet_audit(&mut conn, wallet).await
        {
            Ok(audit) => HttpResponse::Ok().json(audit),
            Err(_) => HttpResponse::InternalServerError().body("Failed to load wallet audit"),
        },
        Ok(None) => wallet_not_linked_response(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to load wallet"),
    }
}

async fn link_user_wallet_by_id(
    pool: web::Data<db::DbPool>,
    requester_id: i32,
    target_user_id: i32,
) -> HttpResponse {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    if let Err(response) = ensure_user_wallet_access(
        &mut conn,
        requester_id,
        target_user_id,
        WalletOperation::Link,
    )
    .await
    {
        return response;
    }
    if let Err(response) = ensure_user_exists(&mut conn, target_user_id).await {
        return response;
    }

    match wallet_service::link_user_wallet(&mut conn, target_user_id).await {
        Ok(linked_wallet) => link_response(linked_wallet),
        Err(_) => HttpResponse::InternalServerError().body("Failed to link wallet"),
    }
}

async fn get_organization_wallet(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_organization_wallet_by_id(pool, requester.user_id, path.into_inner()).await
}

async fn get_organization_wallet_audit(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_organization_wallet_audit_by_id(pool, requester.user_id, path.into_inner()).await
}

async fn link_organization_wallet(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    link_organization_wallet_by_id(pool, requester.user_id, path.into_inner()).await
}

async fn get_organization_wallet_by_id(
    pool: web::Data<db::DbPool>,
    requester_id: i32,
    organization_id: i32,
) -> HttpResponse {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    if let Err(response) = ensure_organization_exists(&mut conn, organization_id).await {
        return response;
    }
    if let Err(response) = ensure_organization_wallet_access(
        &mut conn,
        requester_id,
        organization_id,
        WalletOperation::View,
    )
    .await
    {
        return response;
    }

    match wallet_service::find_organization_wallet(&mut conn, organization_id).await {
        Ok(Some(wallet)) => HttpResponse::Ok().json(WalletResponse::from(&wallet)),
        Ok(None) => wallet_not_linked_response(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to load wallet"),
    }
}

async fn get_organization_wallet_audit_by_id(
    pool: web::Data<db::DbPool>,
    requester_id: i32,
    organization_id: i32,
) -> HttpResponse {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    if let Err(response) = ensure_organization_exists(&mut conn, organization_id).await {
        return response;
    }
    if let Err(response) = ensure_organization_wallet_access(
        &mut conn,
        requester_id,
        organization_id,
        WalletOperation::View,
    )
    .await
    {
        return response;
    }

    match wallet_service::find_organization_wallet(&mut conn, organization_id).await {
        Ok(Some(wallet)) => match wallet_audit_service::build_wallet_audit(&mut conn, wallet).await
        {
            Ok(audit) => HttpResponse::Ok().json(audit),
            Err(_) => HttpResponse::InternalServerError().body("Failed to load wallet audit"),
        },
        Ok(None) => wallet_not_linked_response(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to load wallet"),
    }
}

async fn link_organization_wallet_by_id(
    pool: web::Data<db::DbPool>,
    requester_id: i32,
    organization_id: i32,
) -> HttpResponse {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    if let Err(response) = ensure_organization_exists(&mut conn, organization_id).await {
        return response;
    }
    if let Err(response) = ensure_organization_wallet_access(
        &mut conn,
        requester_id,
        organization_id,
        WalletOperation::Link,
    )
    .await
    {
        return response;
    }

    match wallet_service::link_organization_wallet(&mut conn, organization_id).await {
        Ok(linked_wallet) => link_response(linked_wallet),
        Err(_) => HttpResponse::InternalServerError().body("Failed to link wallet"),
    }
}

pub fn wallet_scope() -> actix_web::Scope {
    web::scope("/wallets")
        .service(web::resource("/me").route(web::get().to(get_my_wallet)))
        .service(web::resource("/me/audit").route(web::get().to(get_my_wallet_audit)))
        .service(web::resource("/me/link").route(web::post().to(link_my_wallet)))
        .service(web::resource("/users/{id}").route(web::get().to(get_user_wallet)))
        .service(web::resource("/users/{id}/audit").route(web::get().to(get_user_wallet_audit)))
        .service(web::resource("/users/{id}/link").route(web::post().to(link_user_wallet)))
        .service(web::resource("/organizations/{id}").route(web::get().to(get_organization_wallet)))
        .service(
            web::resource("/organizations/{id}/audit")
                .route(web::get().to(get_organization_wallet_audit)),
        )
        .service(
            web::resource("/organizations/{id}/link")
                .route(web::post().to(link_organization_wallet)),
        )
}
