pub mod dto;
mod handlers;
mod routes;

pub use handlers::{get_notification_preferences, save_notification_preferences};
pub use routes::configure_routes;
