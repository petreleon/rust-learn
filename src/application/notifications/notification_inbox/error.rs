#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationInboxError {
    Connection(String),
    Database(String),
}
