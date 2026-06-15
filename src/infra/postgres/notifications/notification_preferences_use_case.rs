use futures::future::{BoxFuture, FutureExt};

use crate::application::notifications::preference_service::NotificationPreferencesUseCase;
use crate::application::notifications::preferences::{
    NotificationPreferenceOutput, NotificationPreferencesError,
};
use crate::application::notifications::save_preferences::SaveNotificationPreferencesCommand;
use crate::application::notifications::{get_preferences, save_preferences};
use crate::infra::postgres::notifications::notification_preference_store::PostgresNotificationPreferenceStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresNotificationPreferencesUseCase {
    pool: DbPool,
}

impl PostgresNotificationPreferencesUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl NotificationPreferencesUseCase for PostgresNotificationPreferencesUseCase {
    fn get_preferences(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<NotificationPreferenceOutput, NotificationPreferencesError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresNotificationPreferenceStore::new(&mut conn);
            get_preferences::get_preferences(&mut store, user_id).await
        }
        .boxed()
    }

    fn save_preferences(
        &self,
        command: SaveNotificationPreferencesCommand,
    ) -> BoxFuture<'_, Result<NotificationPreferenceOutput, NotificationPreferencesError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresNotificationPreferenceStore::new(&mut conn);
            save_preferences::save_preferences(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresNotificationPreferencesUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        NotificationPreferencesError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| NotificationPreferencesError::Connection(error.to_string()))
    }
}
