use actix_web::web;

// TODO(level-2): delete this compatibility wrapper when course routes compose
// `http::content::routes::configure_content_item_routes` directly.
pub fn config(cfg: &mut web::ServiceConfig) {
    crate::http::content::routes::configure_content_item_routes(cfg);
}
