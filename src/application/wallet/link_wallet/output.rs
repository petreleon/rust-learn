use crate::application::wallet::wallet_view::WalletView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedWalletView {
    pub wallet: WalletView,
    pub created: bool,
}
