use serde::Serialize;

use crate::application::wallet::wallet_view::WalletView;

#[derive(Debug, Clone, Serialize)]
pub struct WalletResponse {
    pub id: i32,
    pub owner_type: &'static str,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}

impl From<WalletView> for WalletResponse {
    fn from(wallet: WalletView) -> Self {
        Self {
            id: wallet.id,
            owner_type: wallet.owner_type.as_str(),
            user_id: wallet.user_id,
            organization_id: wallet.organization_id,
            value: wallet.value,
        }
    }
}
