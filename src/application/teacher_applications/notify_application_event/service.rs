use futures::future::BoxFuture;

use super::{
    TeacherApplicationNotificationCommand, TeacherApplicationNotificationError,
    TeacherApplicationNotificationOutcome,
};

pub trait TeacherApplicationNotificationUseCase: Send + Sync {
    fn notify_application_event(
        &self,
        command: TeacherApplicationNotificationCommand,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationNotificationOutcome, TeacherApplicationNotificationError>,
    >;
}
