pub fn wallet_scope() -> actix_web::Scope {
    web::scope("/wallets")
        .configure(crate::http::wallet::configure_routes)
        .service(web::resource("/me/deposits").route(web::post().to(deposit_my_tokens)))
        .service(web::resource("/me/retirements").route(web::post().to(retire_my_tokens)))
}
