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

fn wallet_token_transfer_error_response(error: WalletTokenTransferError) -> HttpResponse {
    match error {
        WalletTokenTransferError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have wallet tax permission")
        }
        WalletTokenTransferError::KycRequired => {
            HttpResponse::Conflict().body("KYC verification is required before wallet actions")
        }
        WalletTokenTransferError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        WalletTokenTransferError::InsufficientFunds => {
            HttpResponse::Conflict().body("Insufficient wallet balance")
        }
        WalletTokenTransferError::Database(message) => {
            log::error!("event=wallet_token_transfer_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process wallet token transfer")
        }
    }
}

async fn get_my_wallet(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_user_wallet_by_id(pool, requester.user_id, requester.user_id).await
}

async fn get_my_wallet_audit(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_user_wallet_audit_by_id(pool, requester.user_id, requester.user_id).await
}

async fn link_my_wallet(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    link_user_wallet_by_id(pool, requester.user_id, requester.user_id).await
}

async fn list_wallet_token_taxes(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    if let Err(response) = authenticated_user(&req) {
        return response;
    }
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match wallet_service::get_wallet_token_taxes(&mut conn).await {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(error) => wallet_token_transfer_error_response(error),
    }
}
