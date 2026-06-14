use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::verify_email::{
    self, VerifyEmailCommand, VerifyEmailError, VerifyEmailOutcome, VerifyEmailUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::identity::verify_email_store::PostgresVerifyEmailStore;

#[derive(Clone)]
pub struct PostgresVerifyEmailUseCase {
    pool: DbPool,
}

impl PostgresVerifyEmailUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl VerifyEmailUseCase for PostgresVerifyEmailUseCase {
    fn verify_email(
        &self,
        command: VerifyEmailCommand,
    ) -> BoxFuture<'_, Result<VerifyEmailOutcome, VerifyEmailError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresVerifyEmailStore::new(&mut conn);
            verify_email::verify_email(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresVerifyEmailUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        VerifyEmailError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| VerifyEmailError::Connection(error.to_string()))
    }
}
