use crate::application::wallet::wallet_view::WalletView;
use crate::application::wallet::{wallet_view_output, WalletViewFact};
use crate::models::wallet::Wallet;

pub(super) fn wallet_view_output_from_record(wallet: Wallet) -> WalletView {
    wallet_view_output(WalletViewFact {
        id: wallet.id,
        user_id: wallet.user_id,
        organization_id: wallet.organization_id,
        value: wallet.value.to_string(),
    })
}
