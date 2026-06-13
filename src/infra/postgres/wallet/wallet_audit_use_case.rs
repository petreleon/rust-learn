use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::audit_wallet::{
    self, WalletAudit, WalletAuditError, WalletAuditTarget, WalletAuditUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::wallet::wallet_audit_store::PostgresWalletAuditStore;

#[derive(Clone)]
pub struct PostgresWalletAuditUseCase {
    pool: DbPool,
}

impl PostgresWalletAuditUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl WalletAuditUseCase for PostgresWalletAuditUseCase {
    fn audit_wallet(
        &self,
        target: WalletAuditTarget,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletAuditStore::new(&mut conn);
            audit_wallet::audit_wallet(&mut store, target).await
        }
        .boxed()
    }
}

impl PostgresWalletAuditUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, WalletAuditError> {
        self.pool
            .get()
            .await
            .map_err(|error| WalletAuditError::Connection(error.to_string()))
    }
}
