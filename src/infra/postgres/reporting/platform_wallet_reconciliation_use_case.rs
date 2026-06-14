use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_wallet_reconciliation::{
    self, PlatformWalletReconciliationError, PlatformWalletReconciliationOutput,
    PlatformWalletReconciliationUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_store::PostgresPlatformWalletReconciliationStore;

#[derive(Clone)]
pub struct PostgresPlatformWalletReconciliationUseCase {
    pool: DbPool,
}

impl PostgresPlatformWalletReconciliationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PlatformWalletReconciliationUseCase for PostgresPlatformWalletReconciliationUseCase {
    fn load_platform_wallet_reconciliation(
        &self,
    ) -> BoxFuture<'_, Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformWalletReconciliationStore::new(&mut conn);
            platform_wallet_reconciliation::load_platform_wallet_reconciliation(&mut store).await
        }
        .boxed()
    }
}

impl PostgresPlatformWalletReconciliationUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        PlatformWalletReconciliationError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| PlatformWalletReconciliationError::Connection(error.to_string()))
    }
}
