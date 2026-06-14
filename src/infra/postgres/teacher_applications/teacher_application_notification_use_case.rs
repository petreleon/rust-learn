use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::notify_application_event::{
    self, TeacherApplicationNotificationCommand, TeacherApplicationNotificationError,
    TeacherApplicationNotificationOutcome, TeacherApplicationNotificationUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_notification_store::PostgresTeacherApplicationNotificationStore;
use crate::utils::notifications::NotificationsState;

#[derive(Clone)]
pub struct PostgresTeacherApplicationNotificationUseCase {
    pool: DbPool,
    notifications: NotificationsState,
}

impl PostgresTeacherApplicationNotificationUseCase {
    pub fn new(pool: DbPool, notifications: NotificationsState) -> Self {
        Self {
            pool,
            notifications,
        }
    }
}

impl TeacherApplicationNotificationUseCase for PostgresTeacherApplicationNotificationUseCase {
    fn notify_application_event(
        &self,
        command: TeacherApplicationNotificationCommand,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationNotificationOutcome, TeacherApplicationNotificationError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationNotificationStore::new(
                &mut conn,
                self.notifications.clone(),
            );
            notify_application_event::notify_application_event(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationNotificationUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        TeacherApplicationNotificationError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationNotificationError::Connection(error.to_string()))
    }
}
