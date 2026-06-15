use futures::future::{BoxFuture, FutureExt};

use crate::application::notifications::notification_inbox::{
    self, NotificationInboxError, NotificationInboxUseCase, NotificationOutput,
};
use crate::infra::postgres::notifications::notification_inbox_store::PostgresNotificationInboxStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresNotificationInboxUseCase {
    pool: DbPool,
}

impl PostgresNotificationInboxUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl NotificationInboxUseCase for PostgresNotificationInboxUseCase {
    fn list_notifications(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<NotificationOutput>, NotificationInboxError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresNotificationInboxStore::new(&mut conn);
            notification_inbox::list_notifications(&mut store, user_id).await
        }
        .boxed()
    }

    fn mark_notification_read(
        &self,
        user_id: i32,
        notification_id: i64,
    ) -> BoxFuture<'_, Result<(), NotificationInboxError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresNotificationInboxStore::new(&mut conn);
            notification_inbox::mark_notification_read(&mut store, user_id, notification_id).await
        }
        .boxed()
    }

    fn clear_notifications(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<(), NotificationInboxError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresNotificationInboxStore::new(&mut conn);
            notification_inbox::clear_notifications(&mut store, user_id).await
        }
        .boxed()
    }
}

impl PostgresNotificationInboxUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        NotificationInboxError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| NotificationInboxError::Connection(error.to_string()))
    }
}
