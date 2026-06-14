use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::application::wallet::manage_token_tax::{WalletTokenTaxSettings, WalletTokenTaxView};

#[derive(Debug, Clone, Deserialize)]
pub struct SetWalletTokenTaxRequest {
    pub tax_amount: BigDecimal,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WalletTokenTaxResponse {
    pub operation: String,
    pub tax_amount: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WalletTokenTaxSettingsResponse {
    pub deposit: WalletTokenTaxResponse,
    pub retire: WalletTokenTaxResponse,
}

impl From<WalletTokenTaxView> for WalletTokenTaxResponse {
    fn from(view: WalletTokenTaxView) -> Self {
        Self {
            operation: view.operation.to_string(),
            tax_amount: view.tax_amount,
        }
    }
}

impl From<WalletTokenTaxSettings> for WalletTokenTaxSettingsResponse {
    fn from(settings: WalletTokenTaxSettings) -> Self {
        Self {
            deposit: WalletTokenTaxResponse::from(settings.deposit),
            retire: WalletTokenTaxResponse::from(settings.retire),
        }
    }
}
