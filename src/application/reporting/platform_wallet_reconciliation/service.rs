use futures::future::BoxFuture;

use crate::application::reporting::platform_wallet_reconciliation::{
    PlatformWalletReconciliationError, PlatformWalletReconciliationOutput,
};

pub trait PlatformWalletReconciliationUseCase: Send + Sync {
    fn load_platform_wallet_reconciliation(
        &self,
    ) -> BoxFuture<'_, Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError>>;
}
