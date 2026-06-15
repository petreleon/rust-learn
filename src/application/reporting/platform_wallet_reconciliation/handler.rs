use crate::application::reporting::platform_wallet_reconciliation::store::PlatformWalletReconciliationStore;
use crate::application::reporting::platform_wallet_reconciliation::{
    PlatformWalletReconciliationError, PlatformWalletReconciliationOutput,
};

pub async fn load_platform_wallet_reconciliation(
    store: &mut impl PlatformWalletReconciliationStore,
) -> Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError> {
    store.load_platform_wallet_reconciliation().await
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::load_platform_wallet_reconciliation;
    use crate::application::reporting::platform_wallet_reconciliation::store::PlatformWalletReconciliationStore;
    use crate::application::reporting::platform_wallet_reconciliation::{
        PlatformWalletReconciliationError, PlatformWalletReconciliationOutput,
        PlatformWalletReconciliationRowOutput,
    };
    use crate::domain::wallet::owner::WalletOwnerType;

    #[test]
    fn loads_platform_wallet_reconciliation_through_store_port() {
        let mut store = FakePlatformWalletReconciliationStore { called: false };

        let output = block_on(load_platform_wallet_reconciliation(&mut store))
            .expect("platform wallet reconciliation should load");

        assert!(store.called);
        assert_eq!(output.total_wallets, 1);
        assert_eq!(output.total_reward_records, 3);
        assert_eq!(output.wallets[0].missing_credit_count, 1);
    }

    struct FakePlatformWalletReconciliationStore {
        called: bool,
    }

    impl PlatformWalletReconciliationStore for FakePlatformWalletReconciliationStore {
        fn load_platform_wallet_reconciliation(
            &mut self,
        ) -> BoxFuture<
            '_,
            Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError>,
        > {
            self.called = true;
            ready(Ok(PlatformWalletReconciliationOutput {
                total_wallets: 1,
                total_internal_transactions: 2,
                total_external_transactions: 2,
                total_reward_records: 3,
                total_needs_reconciliation: 0,
                wallets: vec![PlatformWalletReconciliationRowOutput {
                    wallet_id: 10,
                    owner_type: WalletOwnerType::User,
                    user_id: Some(20),
                    organization_id: None,
                    balance: "50".to_string(),
                    internal_transaction_count: 2,
                    external_transaction_count: 2,
                    reward_record_count: 3,
                    needs_reconciliation_count: 0,
                    missing_credit_count: 1,
                    missing_notification_count: 0,
                    missing_payout_count: 0,
                }],
            }))
            .boxed()
        }
    }
}
