use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::login::{
    self, LoginCommand, LoginError, LoginOutput, LoginUseCase,
};
use crate::infra::postgres::identity::login_security::{
    BcryptPasswordVerifier, JwtLoginTokenIssuer,
};
use crate::infra::postgres::identity::login_store::PostgresLoginStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresLoginUseCase {
    pool: DbPool,
}

impl PostgresLoginUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl LoginUseCase for PostgresLoginUseCase {
    fn login(&self, command: LoginCommand) -> BoxFuture<'_, Result<LoginOutput, LoginError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresLoginStore::new(&mut conn);
            login::login(
                &mut store,
                &BcryptPasswordVerifier,
                &JwtLoginTokenIssuer,
                command,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresLoginUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        LoginError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| LoginError::Connection(error.to_string()))
    }
}
