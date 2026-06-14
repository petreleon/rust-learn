mod error;
mod event;
mod handler;
mod output;
mod service;
mod store;

#[cfg(test)]
mod handler_tests;
#[cfg(test)]
pub(crate) mod test_support;

pub use error::WalletDepositIndexError;
pub use event::ObservedWalletDepositEvent;
pub use handler::index_observed_deposit;
pub use output::WalletDepositIndexOutput;
pub use service::WalletDepositIndexUseCase;
pub use store::WalletDepositIndexStore;
