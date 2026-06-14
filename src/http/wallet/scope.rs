use actix_web::{web, Scope};

use crate::http::wallet::routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(wallets_scope());
}

fn wallets_scope() -> Scope {
    web::scope("/wallets").configure(routes::configure_wallet_routes)
}
