pub mod dto;
mod errors;
mod handlers;
mod routes;

pub use handlers::{
    clear_notifications, get_notification_preferences, list_notifications, mark_notification_read,
    save_notification_preferences,
};
pub use routes::configure_routes;
