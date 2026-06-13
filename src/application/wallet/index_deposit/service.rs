use futures::future::BoxFuture;

use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexError, WalletDepositIndexOutput,
};

pub trait WalletDepositIndexUseCase: Send + Sync {
    fn index_observed_deposit(
        &self,
        event: ObservedWalletDepositEvent,
    ) -> BoxFuture<'_, Result<WalletDepositIndexOutput, WalletDepositIndexError>>;
}
