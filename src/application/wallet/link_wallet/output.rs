use crate::application::wallet::wallet_view::WalletView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedWalletView {
    pub wallet: WalletView,
    pub created: bool,
}

pub(crate) struct LinkedWalletFact {
    pub wallet: WalletView,
    pub created: bool,
}

pub(crate) fn linked_wallet_view(fact: LinkedWalletFact) -> LinkedWalletView {
    LinkedWalletView {
        wallet: fact.wallet,
        created: fact.created,
    }
}
