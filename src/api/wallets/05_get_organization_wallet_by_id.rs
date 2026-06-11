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
        .service(web::resource("/me/deposits").route(web::post().to(deposit_my_tokens)))
        .service(web::resource("/me/retirements").route(web::post().to(retire_my_tokens)))
        .service(web::resource("/token-taxes").route(web::get().to(list_wallet_token_taxes)))
        .service(web::resource("/token-taxes/deposit").route(web::put().to(set_deposit_tax)))
        .service(web::resource("/token-taxes/retire").route(web::put().to(set_retire_tax)))
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
