mod authentication;
pub mod dto;
mod handlers;
mod platform_role_assignment;
mod routes;
mod user_handlers;
mod user_routes;

pub use authentication::{auth_scope, jwks, validate_password_strength};
pub use handlers::get_current_session;
pub use routes::configure_routes;
