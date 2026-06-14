use serde::Serialize;

use crate::application::wallet::link_wallet::LinkedWalletView;
use crate::http::wallet::dto::WalletResponse;

#[derive(Debug, Clone, Serialize)]
pub struct WalletLinkResponse {
    pub wallet: WalletResponse,
    pub created: bool,
}

impl From<LinkedWalletView> for WalletLinkResponse {
    fn from(linked_wallet: LinkedWalletView) -> Self {
        Self {
            wallet: WalletResponse::from(linked_wallet.wallet),
            created: linked_wallet.created,
        }
    }
}
