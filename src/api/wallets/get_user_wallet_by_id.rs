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
    if let Err(response) = ensure_user_kyc_verified(&mut conn, target_user_id).await {
        return response;
    }

    match wallet_service::link_user_wallet(&mut conn, target_user_id).await {
        Ok(linked_wallet) => link_response(linked_wallet),
        Err(_) => HttpResponse::InternalServerError().body("Failed to link wallet"),
    }
}

async fn ensure_user_kyc_verified(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<(), HttpResponse> {
    match users::table
        .find(user_id)
        .select(users::kyc_verified)
        .first::<bool>(conn)
        .await
    {
        Ok(true) => Ok(()),
        Ok(false) => {
            Err(HttpResponse::Conflict().body("KYC verification is required before wallet actions"))
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to load user KYC status")),
    }
}

async fn get_organization_wallet(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    get_organization_wallet_by_id(pool, requester.user_id, path.into_inner()).await
}

async fn link_organization_wallet(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    link_organization_wallet_by_id(pool, requester.user_id, path.into_inner()).await
}
