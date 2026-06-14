mod messages;
mod mutations;
mod senders;
mod state;
mod teacher_application;

pub use messages::{
    content_published_notification, enrollment_notification, reward_event_notification,
    reward_wallet_credit_notification, role_assignment_notification, worker_failure_notification,
    NotificationMessage,
};
pub use mutations::{create_notification, create_notifications_bulk};
pub use state::NotificationsState;
pub use teacher_application::teacher_application_notification;

#[cfg(test)]
mod tests;
