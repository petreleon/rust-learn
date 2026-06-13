mod action;
mod error;
mod handler;
mod service;
mod store;

#[cfg(test)]
mod handler_tests;
#[cfg(test)]
pub(crate) mod test_support;

pub use action::WalletAuthorizationAction;
pub use error::WalletAuthorizationError;
pub use handler::authorize_wallet_action;
pub use service::WalletAuthorizationUseCase;
pub use store::WalletAuthorizationStore;
