use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::audit_wallet::{
    self, WalletAudit, WalletAuditError, WalletAuditSubject, WalletAuditUseCase,
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
        actor_user_id: i32,
        subject: WalletAuditSubject,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresWalletAuditStore::new(&mut conn);
            audit_wallet::audit_wallet_for_actor(&mut store, actor_user_id, subject).await
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
