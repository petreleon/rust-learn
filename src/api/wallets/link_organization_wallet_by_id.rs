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
        .configure(crate::http::wallet::configure_routes)
        .service(web::resource("/me/link").route(web::post().to(link_my_wallet)))
        .service(web::resource("/me/deposits").route(web::post().to(deposit_my_tokens)))
        .service(web::resource("/me/retirements").route(web::post().to(retire_my_tokens)))
        .service(web::resource("/token-taxes").route(web::get().to(list_wallet_token_taxes)))
        .service(web::resource("/token-taxes/deposit").route(web::put().to(set_deposit_tax)))
        .service(web::resource("/token-taxes/retire").route(web::put().to(set_retire_tax)))
        .service(web::resource("/users/{id}/link").route(web::post().to(link_user_wallet)))
        .service(
            web::resource("/organizations/{id}/link")
                .route(web::post().to(link_organization_wallet)),
        )
}
