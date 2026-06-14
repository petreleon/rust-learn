use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_csv_exports::store::PlatformCsvExportStore;
use crate::application::reporting::platform_csv_exports::*;
use crate::infra::postgres::reporting::{
    platform_csv_export_delegated_permissions, platform_csv_export_reward_approvals,
    platform_csv_export_teacher_applications, platform_csv_export_token_payouts,
    platform_csv_export_wallet_credits,
};

pub struct PostgresPlatformCsvExportStore<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> PostgresPlatformCsvExportStore<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformCsvExportStore for PostgresPlatformCsvExportStore<'_> {
    fn load_platform_teacher_application_exports(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<PlatformTeacherApplicationExportRowOutput>, PlatformCsvExportError>>
    {
        async move {
            platform_csv_export_teacher_applications::load_teacher_application_export_rows(
                self.conn,
            )
            .await
        }
        .boxed()
    }

    fn load_platform_reward_approval_exports(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardApprovalExportRowOutput>, PlatformCsvExportError>>
    {
        async move {
            platform_csv_export_reward_approvals::load_reward_approval_export_rows(self.conn).await
        }
        .boxed()
    }

    fn load_platform_token_payout_exports(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<PlatformTokenPayoutExportRowOutput>, PlatformCsvExportError>>
    {
        async move { platform_csv_export_token_payouts::load_token_payout_export_rows(self.conn).await }
            .boxed()
    }

    fn load_platform_wallet_credit_exports(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<PlatformWalletCreditExportRowOutput>, PlatformCsvExportError>>
    {
        async move {
            platform_csv_export_wallet_credits::load_wallet_credit_export_rows(self.conn).await
        }
        .boxed()
    }

    fn load_platform_delegated_permission_exports(
        &mut self,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformDelegatedPermissionExportRowOutput>, PlatformCsvExportError>,
    > {
        async move {
            platform_csv_export_delegated_permissions::load_delegated_permission_export_rows(
                self.conn,
            )
            .await
        }
        .boxed()
    }
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> PlatformCsvExportError {
    PlatformCsvExportError::Database(error.to_string())
}
