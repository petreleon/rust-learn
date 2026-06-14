#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationNotificationError {
    Connection(String),
    Database(String),
    Delivery(String),
}
