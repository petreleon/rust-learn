use bigdecimal::BigDecimal;
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::manage_token_tax::{
    self, WalletTokenTaxAuditEventView, WalletTokenTaxError, WalletTokenTaxOperation,
    WalletTokenTaxSettings, WalletTokenTaxUseCase, WalletTokenTaxView,
};
use crate::infra::postgres::wallet::wallet_token_tax_store::PostgresWalletTokenTaxStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresWalletTokenTaxUseCase {
    pool: DbPool,
}

impl PostgresWalletTokenTaxUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WalletTokenTaxUseCase for PostgresWalletTokenTaxUseCase {
    fn list_token_taxes(
        &self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<WalletTokenTaxSettings, WalletTokenTaxError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletTokenTaxStore::new(&mut conn);
            manage_token_tax::list_token_taxes(&mut store, actor_user_id).await
        }
        .boxed()
    }

    fn list_token_tax_audit(
        &self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<WalletTokenTaxAuditEventView>, WalletTokenTaxError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletTokenTaxStore::new(&mut conn);
            manage_token_tax::list_token_tax_audit(&mut store, actor_user_id).await
        }
        .boxed()
    }

    fn set_token_tax(
        &self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<WalletTokenTaxView, WalletTokenTaxError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletTokenTaxStore::new(&mut conn);
            manage_token_tax::set_token_tax(&mut store, actor_user_id, operation, amount).await
        }
        .boxed()
    }
}

impl PostgresWalletTokenTaxUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, WalletTokenTaxError> {
        self.pool
            .get()
            .await
            .map_err(|error| WalletTokenTaxError::Connection(error.to_string()))
    }
}
