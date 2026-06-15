use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::retire_tokens::{
    self, WalletRetirementCommand, WalletRetirementError, WalletRetirementUseCase,
    WalletRetirementView,
};
use crate::db::DbPool;
use crate::infra::postgres::wallet::wallet_retirement_store::PostgresWalletRetirementStore;

#[derive(Clone)]
pub struct PostgresWalletRetirementUseCase {
    pool: DbPool,
}

impl PostgresWalletRetirementUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WalletRetirementUseCase for PostgresWalletRetirementUseCase {
    fn retire_tokens(
        &self,
        user_id: i32,
        request: WalletRetirementCommand,
    ) -> BoxFuture<'_, Result<WalletRetirementView, WalletRetirementError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletRetirementStore::new(&mut conn);
            retire_tokens::retire_tokens(&mut store, user_id, request).await
        }
        .boxed()
    }
}

impl PostgresWalletRetirementUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, WalletRetirementError> {
        self.pool
            .get()
            .await
            .map_err(|error| WalletRetirementError::Connection(error.to_string()))
    }
}
