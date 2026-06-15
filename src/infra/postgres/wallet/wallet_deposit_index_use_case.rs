use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::index_deposit::{
    self, ObservedWalletDepositEvent, WalletDepositIndexError, WalletDepositIndexOutput,
    WalletDepositIndexUseCase,
};
use crate::infra::postgres::wallet::wallet_deposit_index_store::PostgresWalletDepositIndexStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresWalletDepositIndexUseCase {
    pool: DbPool,
}

impl PostgresWalletDepositIndexUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WalletDepositIndexUseCase for PostgresWalletDepositIndexUseCase {
    fn index_observed_deposit(
        &self,
        event: ObservedWalletDepositEvent,
    ) -> BoxFuture<'_, Result<WalletDepositIndexOutput, WalletDepositIndexError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletDepositIndexStore::new(&mut conn);
            index_deposit::index_observed_deposit(&mut store, event).await
        }
        .boxed()
    }
}

impl PostgresWalletDepositIndexUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, WalletDepositIndexError> {
        self.pool
            .get()
            .await
            .map_err(|error| WalletDepositIndexError::Connection(error.to_string()))
    }
}
