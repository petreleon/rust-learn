use futures::future::BoxFuture;

use super::{TeacherApplicationNotificationCommand, TeacherApplicationNotificationError};

pub trait TeacherApplicationNotificationStore {
    fn platform_reviewer_ids(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<i32>, TeacherApplicationNotificationError>>;

    fn organization_viewer_ids(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, TeacherApplicationNotificationError>>;

    fn send(
        &mut self,
        recipient_user_id: i32,
        command: TeacherApplicationNotificationCommand,
    ) -> BoxFuture<'_, Result<(), TeacherApplicationNotificationError>>;
}
