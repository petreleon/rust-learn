use crate::application::wallet::read_wallet::WalletView;
use crate::models::wallet::Wallet;

pub(super) fn wallet_view_from_model(wallet: Wallet) -> WalletView {
    let owner_type = if wallet.user_id.is_some() {
        "user"
    } else {
        "organization"
    };

    WalletView {
        id: wallet.id,
        owner_type,
        user_id: wallet.user_id,
        organization_id: wallet.organization_id,
        value: wallet.value.to_string(),
    }
}
