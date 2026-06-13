use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::read_wallet::{
    self, WalletReadError, WalletReadSubject, WalletReadUseCase, WalletView,
};
use crate::db::DbPool;
use crate::infra::postgres::wallet::wallet_read_store::PostgresWalletReadStore;

#[derive(Clone)]
pub struct PostgresWalletReadUseCase {
    pool: DbPool,
}

impl PostgresWalletReadUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WalletReadUseCase for PostgresWalletReadUseCase {
    fn read_wallet(
        &self,
        actor_user_id: i32,
        subject: WalletReadSubject,
    ) -> BoxFuture<'_, Result<WalletView, WalletReadError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletReadStore::new(&mut conn);
            read_wallet::read_wallet(&mut store, actor_user_id, subject).await
        }
        .boxed()
    }
}

impl PostgresWalletReadUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, WalletReadError> {
        self.pool
            .get()
            .await
            .map_err(|error| WalletReadError::Connection(error.to_string()))
    }
}
