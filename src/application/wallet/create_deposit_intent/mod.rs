mod draft;
mod error;
mod gas_payer;
mod handler;
mod input;
mod output;
mod service;
mod store;

#[cfg(test)]
mod handler_tests;
#[cfg(test)]
pub(crate) mod test_support;

pub use draft::WalletDepositIntentDraft;
pub use error::WalletDepositIntentError;
pub use gas_payer::WalletDepositGasPayer;
pub use handler::create_deposit_intent;
pub use input::WalletDepositIntentRequest;
pub use output::WalletDepositIntentView;
pub use service::WalletDepositIntentUseCase;
pub use store::WalletDepositIntentStore;
