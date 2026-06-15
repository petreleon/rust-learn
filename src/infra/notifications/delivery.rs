use futures::future::{BoxFuture, FutureExt};

use crate::application::notifications::delivery::{
    ContentPublishedNotification, EnrollmentNotificationCommand, NotificationDeliveryError,
    NotificationDeliveryUseCase, RoleAssignmentNotification,
};

use super::NotificationsState;

fn delivery_error(error: anyhow::Error) -> NotificationDeliveryError {
    NotificationDeliveryError::new(error.to_string())
}

impl NotificationDeliveryUseCase for NotificationsState {
    fn send_content_published(
        &self,
        notification: ContentPublishedNotification,
    ) -> BoxFuture<'_, Result<i64, NotificationDeliveryError>> {
        async move {
            self.send_content_published_notification(
                notification.recipient_user_id,
                notification.course_id,
                notification.content_id,
                notification.content_type,
            )
            .await
            .map_err(delivery_error)
        }
        .boxed()
    }

    fn send_enrollment(
        &self,
        notification: EnrollmentNotificationCommand,
    ) -> BoxFuture<'_, Result<i64, NotificationDeliveryError>> {
        async move {
            self.send_enrollment_notification(
                notification.target_user_id,
                notification.course_id,
                notification.course_title,
            )
            .await
            .map_err(delivery_error)
        }
        .boxed()
    }

    fn send_role_assignment(
        &self,
        notification: RoleAssignmentNotification,
    ) -> BoxFuture<'_, Result<i64, NotificationDeliveryError>> {
        async move {
            self.send_role_assignment_notification(
                notification.target_user_id,
                notification.scope.as_str(),
                Some(notification.scope_id),
                notification.role_name,
            )
            .await
            .map_err(delivery_error)
        }
        .boxed()
    }
}
