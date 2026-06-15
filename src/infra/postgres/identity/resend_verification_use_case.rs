use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::resend_verification::{
    self, ResendVerificationCommand, ResendVerificationError, ResendVerificationOutcome,
    ResendVerificationUseCase,
};
use crate::infra::postgres::identity::resend_verification_delivery::{
    GeneratedVerificationToken, MockVerificationEmailSender,
};
use crate::infra::postgres::identity::resend_verification_store::PostgresResendVerificationStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresResendVerificationUseCase {
    pool: DbPool,
}

impl PostgresResendVerificationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ResendVerificationUseCase for PostgresResendVerificationUseCase {
    fn resend_verification(
        &self,
        command: ResendVerificationCommand,
    ) -> BoxFuture<'_, Result<ResendVerificationOutcome, ResendVerificationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresResendVerificationStore::new(&mut conn);
            resend_verification::resend_verification(
                &mut store,
                &GeneratedVerificationToken,
                &MockVerificationEmailSender,
                command,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresResendVerificationUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        ResendVerificationError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| ResendVerificationError::Connection(error.to_string()))
    }
}
