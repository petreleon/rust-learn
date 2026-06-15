mod authentication;
pub mod dto;
mod errors;
mod handlers;
mod platform_role_assignment;
mod routes;
mod user_handlers;
mod user_list;
mod user_routes;

pub use authentication::{auth_scope, jwks};
pub use handlers::get_current_session;
pub use routes::configure_routes;
