use futures::future::BoxFuture;

use crate::application::reporting::platform_csv_exports::{
    PlatformCsvExportError, PlatformDelegatedPermissionExportRowOutput,
    PlatformRewardApprovalExportRowOutput, PlatformTeacherApplicationExportRowOutput,
    PlatformTokenPayoutExportRowOutput, PlatformWalletCreditExportRowOutput,
};

pub trait PlatformCsvExportsUseCase: Send + Sync {
    fn load_platform_teacher_application_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformTeacherApplicationExportRowOutput>, PlatformCsvExportError>>;

    fn load_platform_reward_approval_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardApprovalExportRowOutput>, PlatformCsvExportError>>;

    fn load_platform_token_payout_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformTokenPayoutExportRowOutput>, PlatformCsvExportError>>;

    fn load_platform_wallet_credit_exports(
        &self,
    ) -> BoxFuture<'_, Result<Vec<PlatformWalletCreditExportRowOutput>, PlatformCsvExportError>>;

    fn load_platform_delegated_permission_exports(
        &self,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformDelegatedPermissionExportRowOutput>, PlatformCsvExportError>,
    >;
}
