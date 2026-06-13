async fn set_deposit_tax(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<SetWalletTokenTaxRequest>,
) -> impl Responder {
    set_wallet_token_tax(req, pool, WalletTokenOperation::Deposit, body.into_inner()).await
}

async fn set_retire_tax(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<SetWalletTokenTaxRequest>,
) -> impl Responder {
    set_wallet_token_tax(req, pool, WalletTokenOperation::Retire, body.into_inner()).await
}

async fn set_wallet_token_tax(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    operation: WalletTokenOperation,
    body: SetWalletTokenTaxRequest,
) -> HttpResponse {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match wallet_service::set_wallet_token_tax_for_actor(
        &mut conn,
        requester.user_id,
        operation,
        body,
    )
    .await
    {
        Ok(setting) => HttpResponse::Ok().json(setting),
        Err(error) => wallet_token_transfer_error_response(error),
    }
}

async fn deposit_my_tokens(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<WalletTokenTransferRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match wallet_service::deposit_tokens_to_user_wallet(
        &mut conn,
        requester.user_id,
        body.into_inner(),
    )
    .await
    {
        Ok(result) => HttpResponse::Created().json(result),
        Err(error) => wallet_token_transfer_error_response(error),
    }
}

async fn retire_my_tokens(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<WalletTokenTransferRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match wallet_service::retire_tokens_from_user_wallet(
        &mut conn,
        requester.user_id,
        body.into_inner(),
    )
    .await
    {
        Ok(result) => HttpResponse::Created().json(result),
        Err(error) => wallet_token_transfer_error_response(error),
    }
}
