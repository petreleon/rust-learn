use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::register::{
    self, RegisterCommand, RegisterError, RegisterOutcome, RegisterUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::identity::registration_delivery::{
    GeneratedRegistrationToken, MockRegistrationEmailSender,
};
use crate::infra::postgres::identity::registration_security::BcryptRegistrationPasswordHasher;
use crate::infra::postgres::identity::registration_store::PostgresRegistrationStore;

#[derive(Clone)]
pub struct PostgresRegisterUseCase {
    pool: DbPool,
}

impl PostgresRegisterUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RegisterUseCase for PostgresRegisterUseCase {
    fn register(
        &self,
        command: RegisterCommand,
    ) -> BoxFuture<'_, Result<RegisterOutcome, RegisterError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRegistrationStore::new(&mut conn);
            register::register(
                &mut store,
                &BcryptRegistrationPasswordHasher,
                &GeneratedRegistrationToken,
                &MockRegistrationEmailSender,
                command,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRegisterUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RegisterError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RegisterError::Connection(error.to_string()))
    }
}
