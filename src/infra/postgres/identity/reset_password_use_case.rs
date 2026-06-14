use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::reset_password::{
    self, ResetPasswordCommand, ResetPasswordError, ResetPasswordOutcome, ResetPasswordUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::identity::reset_password_security::BcryptResetPasswordHasher;
use crate::infra::postgres::identity::reset_password_store::PostgresResetPasswordStore;

#[derive(Clone)]
pub struct PostgresResetPasswordUseCase {
    pool: DbPool,
}

impl PostgresResetPasswordUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ResetPasswordUseCase for PostgresResetPasswordUseCase {
    fn reset_password(
        &self,
        command: ResetPasswordCommand,
    ) -> BoxFuture<'_, Result<ResetPasswordOutcome, ResetPasswordError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresResetPasswordStore::new(&mut conn);
            reset_password::reset_password(&mut store, &BcryptResetPasswordHasher, command).await
        }
        .boxed()
    }
}

impl PostgresResetPasswordUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        ResetPasswordError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| ResetPasswordError::Connection(error.to_string()))
    }
}
