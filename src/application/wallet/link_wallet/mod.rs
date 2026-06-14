mod error;
mod handler;
mod output;
mod service;
mod store;
mod subject;

#[cfg(test)]
mod handler_tests;
#[cfg(test)]
pub(crate) mod test_support;

pub use error::WalletLinkError;
pub use handler::link_wallet;
pub use output::LinkedWalletView;
pub use service::WalletLinkUseCase;
pub use store::WalletLinkStore;
pub use subject::WalletLinkSubject;
