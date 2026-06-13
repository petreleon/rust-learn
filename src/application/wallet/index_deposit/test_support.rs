use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexError, WalletDepositIndexOutput,
    WalletDepositIndexStore,
};

pub(crate) struct FakeWalletDepositIndexStore {
    pub event: Option<ObservedWalletDepositEvent>,
    pub output: WalletDepositIndexOutput,
}

impl Default for FakeWalletDepositIndexStore {
    fn default() -> Self {
        Self {
            event: None,
            output: WalletDepositIndexOutput {
                intent_id: Some(10),
                wallet_id: Some(20),
                transaction_id: Some(30),
                external_transaction_id: Some(40),
                internal_transaction_ids: vec![50, 60],
                credited: true,
                status: "credited".to_string(),
            },
        }
    }
}

impl WalletDepositIndexStore for FakeWalletDepositIndexStore {
    fn index_observed_deposit(
        &mut self,
        event: ObservedWalletDepositEvent,
    ) -> BoxFuture<'_, Result<WalletDepositIndexOutput, WalletDepositIndexError>> {
        self.event = Some(event);
        ready(Ok(self.output.clone())).boxed()
    }
}
