use futures::future::BoxFuture;

use crate::application::reporting::platform_wallet_reconciliation::{
    PlatformWalletReconciliationError, PlatformWalletReconciliationOutput,
};

pub trait PlatformWalletReconciliationStore {
    fn load_platform_wallet_reconciliation(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError>>;
}
