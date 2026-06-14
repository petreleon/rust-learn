use futures::future::BoxFuture;

use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexError, WalletDepositIndexOutput,
};

pub trait WalletDepositIndexStore {
    fn index_observed_deposit(
        &mut self,
        event: ObservedWalletDepositEvent,
    ) -> BoxFuture<'_, Result<WalletDepositIndexOutput, WalletDepositIndexError>>;
}
