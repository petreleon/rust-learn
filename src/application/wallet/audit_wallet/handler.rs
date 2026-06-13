use crate::application::wallet::audit_wallet::{
    WalletAudit, WalletAuditError, WalletAuditStore, WalletAuditTarget,
};

pub async fn audit_wallet(
    store: &mut impl WalletAuditStore,
    target: WalletAuditTarget,
) -> Result<WalletAudit, WalletAuditError> {
    store.load_wallet_audit(target).await
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::audit_wallet;
    use crate::application::wallet::audit_wallet::{
        WalletAudit, WalletAuditError, WalletAuditStore, WalletAuditTarget, WalletAuditWallet,
    };

    #[test]
    fn delegates_to_wallet_audit_store() {
        let mut store = FakeWalletAuditStore::default();
        let target = WalletAuditTarget {
            id: 10,
            user_id: Some(7),
            organization_id: None,
            value: "100".to_string(),
        };

        let audit = block_on(audit_wallet(&mut store, target)).expect("wallet audit should load");

        assert!(store.loaded);
        assert_eq!(audit.wallet.id, 10);
        assert_eq!(audit.wallet.owner_type, "user");
    }

    #[derive(Default)]
    struct FakeWalletAuditStore {
        loaded: bool,
    }

    impl WalletAuditStore for FakeWalletAuditStore {
        fn load_wallet_audit(
            &mut self,
            target: WalletAuditTarget,
        ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>> {
            self.loaded = true;
            ready(Ok(WalletAudit {
                wallet: WalletAuditWallet {
                    id: target.id,
                    owner_type: target.owner_type().to_string(),
                    user_id: target.user_id,
                    organization_id: target.organization_id,
                    value: target.value,
                },
                internal_transactions: Vec::new(),
                external_transactions: Vec::new(),
                reward_records: Vec::new(),
                compensation_records: Vec::new(),
            }))
            .boxed()
        }
    }
}
