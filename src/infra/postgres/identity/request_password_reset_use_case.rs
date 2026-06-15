use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::request_password_reset::{
    self, RequestPasswordResetCommand, RequestPasswordResetError, RequestPasswordResetOutcome,
    RequestPasswordResetUseCase,
};
use crate::infra::postgres::identity::request_password_reset_delivery::{
    GeneratedPasswordResetToken, MockPasswordResetEmailSender,
};
use crate::infra::postgres::identity::request_password_reset_store::PostgresRequestPasswordResetStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresRequestPasswordResetUseCase {
    pool: DbPool,
}

impl PostgresRequestPasswordResetUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RequestPasswordResetUseCase for PostgresRequestPasswordResetUseCase {
    fn request_password_reset(
        &self,
        command: RequestPasswordResetCommand,
    ) -> BoxFuture<'_, Result<RequestPasswordResetOutcome, RequestPasswordResetError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRequestPasswordResetStore::new(&mut conn);
            request_password_reset::request_password_reset(
                &mut store,
                &GeneratedPasswordResetToken,
                &MockPasswordResetEmailSender,
                command,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRequestPasswordResetUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RequestPasswordResetError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RequestPasswordResetError::Connection(error.to_string()))
    }
}
