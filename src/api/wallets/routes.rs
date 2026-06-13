pub fn wallet_scope() -> actix_web::Scope {
    web::scope("/wallets")
        .configure(crate::http::wallet::configure_routes)
        .service(web::resource("/me/deposits").route(web::post().to(deposit_my_tokens)))
        .service(web::resource("/me/retirements").route(web::post().to(retire_my_tokens)))
        .service(web::resource("/token-taxes").route(web::get().to(list_wallet_token_taxes)))
        .service(web::resource("/token-taxes/deposit").route(web::put().to(set_deposit_tax)))
        .service(web::resource("/token-taxes/retire").route(web::put().to(set_retire_tax)))
}
