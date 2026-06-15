use futures::future::BoxFuture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentPublishedNotification {
    pub recipient_user_id: i32,
    pub course_id: i32,
    pub content_id: i32,
    pub content_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrollmentNotificationCommand {
    pub target_user_id: i32,
    pub course_id: i32,
    pub course_title: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleAssignmentScope {
    Course,
    Organization,
}

impl RoleAssignmentScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Course => "course",
            Self::Organization => "organization",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleAssignmentNotification {
    pub target_user_id: i32,
    pub scope: RoleAssignmentScope,
    pub scope_id: i32,
    pub role_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryError {
    message: String,
}

impl NotificationDeliveryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub trait NotificationDeliveryUseCase: Send + Sync {
    fn send_content_published(
        &self,
        notification: ContentPublishedNotification,
    ) -> BoxFuture<'_, Result<i64, NotificationDeliveryError>>;

    fn send_enrollment(
        &self,
        notification: EnrollmentNotificationCommand,
    ) -> BoxFuture<'_, Result<i64, NotificationDeliveryError>>;

    fn send_role_assignment(
        &self,
        notification: RoleAssignmentNotification,
    ) -> BoxFuture<'_, Result<i64, NotificationDeliveryError>>;
}
