use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_csv_exports::{
    self, PlatformCsvExportError, PlatformCsvExportsUseCase,
};
use crate::application::reporting::platform_csv_exports::{
    PlatformDelegatedPermissionExportRowOutput, PlatformRewardApprovalExportRowOutput,
    PlatformTeacherApplicationExportRowOutput, PlatformTokenPayoutExportRowOutput,
    PlatformWalletCreditExportRowOutput,
};
use crate::infra::postgres::reporting::platform_csv_export_store::PostgresPlatformCsvExportStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresPlatformCsvExportsUseCase {
    pool: DbPool,
}

impl PostgresPlatformCsvExportsUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PlatformCsvExportsUseCase for PostgresPlatformCsvExportsUseCase {
    fn load_platform_teacher_application_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformTeacherApplicationExportRowOutput>, PlatformCsvExportError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformCsvExportStore::new(&mut conn);
            platform_csv_exports::load_platform_teacher_application_exports(&mut store).await
        }
        .boxed()
    }

    fn load_platform_reward_approval_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardApprovalExportRowOutput>, PlatformCsvExportError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformCsvExportStore::new(&mut conn);
            platform_csv_exports::load_platform_reward_approval_exports(&mut store).await
        }
        .boxed()
    }

    fn load_platform_token_payout_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformTokenPayoutExportRowOutput>, PlatformCsvExportError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformCsvExportStore::new(&mut conn);
            platform_csv_exports::load_platform_token_payout_exports(&mut store).await
        }
        .boxed()
    }

    fn load_platform_wallet_credit_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformWalletCreditExportRowOutput>, PlatformCsvExportError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformCsvExportStore::new(&mut conn);
            platform_csv_exports::load_platform_wallet_credit_exports(&mut store).await
        }
        .boxed()
    }

    fn load_platform_delegated_permission_exports(
        &self,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformDelegatedPermissionExportRowOutput>, PlatformCsvExportError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformCsvExportStore::new(&mut conn);
            platform_csv_exports::load_platform_delegated_permission_exports(&mut store).await
        }
        .boxed()
    }
}

impl PostgresPlatformCsvExportsUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        PlatformCsvExportError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| PlatformCsvExportError::Connection(error.to_string()))
    }
}
