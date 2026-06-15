use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::get_user_profile::{
    self, GetUserProfileCommand, UserProfileReadUseCase,
};
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};
use crate::infra::postgres::identity::user_profile_store::PostgresUserProfileStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresUserProfileReadUseCase {
    pool: DbPool,
}

impl PostgresUserProfileReadUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl UserProfileReadUseCase for PostgresUserProfileReadUseCase {
    fn get_user_profile(
        &self,
        command: GetUserProfileCommand,
    ) -> BoxFuture<'_, Result<UserProfileOutput, UserProfileError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresUserProfileStore::new(&mut conn);
            get_user_profile::get_user_profile(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresUserProfileReadUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        UserProfileError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| UserProfileError::Database(error.to_string()))
    }
}
