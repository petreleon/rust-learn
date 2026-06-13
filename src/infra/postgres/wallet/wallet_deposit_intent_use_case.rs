use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::create_deposit_intent::{
    self, WalletDepositIntentError, WalletDepositIntentRequest, WalletDepositIntentUseCase,
    WalletDepositIntentView,
};
use crate::db::DbPool;
use crate::infra::postgres::wallet::wallet_deposit_intent_store::PostgresWalletDepositIntentStore;

#[derive(Clone)]
pub struct PostgresWalletDepositIntentUseCase {
    pool: DbPool,
}

impl PostgresWalletDepositIntentUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WalletDepositIntentUseCase for PostgresWalletDepositIntentUseCase {
    fn create_deposit_intent(
        &self,
        user_id: i32,
        request: WalletDepositIntentRequest,
    ) -> BoxFuture<'_, Result<WalletDepositIntentView, WalletDepositIntentError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletDepositIntentStore::new(&mut conn);
            create_deposit_intent::create_deposit_intent(&mut store, user_id, request).await
        }
        .boxed()
    }
}

impl PostgresWalletDepositIntentUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, WalletDepositIntentError> {
        self.pool
            .get()
            .await
            .map_err(|error| WalletDepositIntentError::Connection(error.to_string()))
    }
}
