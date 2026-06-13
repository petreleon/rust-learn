use actix_web::web;

pub fn wallet_scope() -> actix_web::Scope {
    web::scope("/wallets").configure(crate::http::wallet::configure_routes)
}
