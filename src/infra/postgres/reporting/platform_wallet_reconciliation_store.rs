use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_wallet_reconciliation::store::PlatformWalletReconciliationStore;
use crate::application::reporting::platform_wallet_reconciliation::{
    PlatformWalletReconciliationError, PlatformWalletReconciliationOutput,
};
use crate::infra::postgres::reporting::platform_wallet_reconciliation_queries;

pub struct PostgresPlatformWalletReconciliationStore<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> PostgresPlatformWalletReconciliationStore<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformWalletReconciliationStore for PostgresPlatformWalletReconciliationStore<'_> {
    fn load_platform_wallet_reconciliation(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError>>
    {
        async move {
            platform_wallet_reconciliation_queries::load_platform_wallet_reconciliation(self.conn)
                .await
        }
        .boxed()
    }
}
