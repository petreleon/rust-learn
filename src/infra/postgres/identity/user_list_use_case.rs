use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::list_users::{self, ListUsersQuery, UserListUseCase};
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};
use crate::db::DbPool;
use crate::infra::postgres::identity::user_profile_store::PostgresUserProfileStore;

#[derive(Clone)]
pub struct PostgresUserListUseCase {
    pool: DbPool,
}

impl PostgresUserListUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl UserListUseCase for PostgresUserListUseCase {
    fn list_users(
        &self,
        query: ListUsersQuery,
    ) -> BoxFuture<'_, Result<Vec<UserProfileOutput>, UserProfileError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresUserProfileStore::new(&mut conn);
            list_users::list_users(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresUserListUseCase {
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
