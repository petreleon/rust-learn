mod course_routes;
pub mod dto;

pub use course_routes::course_scope;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(course_scope());
}
