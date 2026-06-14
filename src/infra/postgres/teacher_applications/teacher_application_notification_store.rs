use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::notify_application_event::{
    TeacherApplicationNotificationCommand, TeacherApplicationNotificationError,
    TeacherApplicationNotificationStore,
};
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::teacher_applications::teacher_application_recipients::{
    list_organization_user_ids_with_permission, list_platform_user_ids_with_permission,
};
use crate::utils::notifications::NotificationsState;

pub struct PostgresTeacherApplicationNotificationStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
    notifications: NotificationsState,
}

impl<'conn> PostgresTeacherApplicationNotificationStore<'conn> {
    pub fn new(
        conn: &'conn mut diesel_async::AsyncPgConnection,
        notifications: NotificationsState,
    ) -> Self {
        Self {
            conn,
            notifications,
        }
    }
}

impl TeacherApplicationNotificationStore for PostgresTeacherApplicationNotificationStore<'_> {
    fn platform_reviewer_ids(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<i32>, TeacherApplicationNotificationError>> {
        async move {
            list_platform_user_ids_with_permission(
                self.conn,
                &Permissions::REVIEW_TEACHER_APPLICATIONS.to_string(),
            )
            .await
            .map_err(map_error)
        }
        .boxed()
    }

    fn organization_viewer_ids(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, TeacherApplicationNotificationError>> {
        async move {
            list_organization_user_ids_with_permission(
                self.conn,
                organization_id,
                &Permissions::VIEW_ORG_TEACHER_APPLICATIONS.to_string(),
            )
            .await
            .map_err(map_error)
        }
        .boxed()
    }

    fn send(
        &mut self,
        recipient_user_id: i32,
        command: TeacherApplicationNotificationCommand,
    ) -> BoxFuture<'_, Result<(), TeacherApplicationNotificationError>> {
        async move {
            self.notifications
                .send_teacher_application_notification(
                    recipient_user_id,
                    command.application_id,
                    command.event_type,
                    command.status,
                    command.requested_scope,
                    command.reason.as_deref(),
                )
                .await
                .map(|_| ())
                .map_err(|error| TeacherApplicationNotificationError::Delivery(error.to_string()))
        }
        .boxed()
    }
}

fn map_error(error: diesel::result::Error) -> TeacherApplicationNotificationError {
    TeacherApplicationNotificationError::Database(error.to_string())
}
