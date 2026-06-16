use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxAuditEventView, WalletTokenTaxSettings, WalletTokenTaxView,
};

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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WalletTokenTaxAuditEventResponse {
    pub id: i64,
    pub actor_user_id: Option<i32>,
    pub operation: String,
    pub previous_tax_amount: String,
    pub new_tax_amount: String,
    pub created_at: DateTime<Utc>,
}

impl From<WalletTokenTaxView> for WalletTokenTaxResponse {
    fn from(view: WalletTokenTaxView) -> Self {
        Self {
            operation: view.operation.to_string(),
            tax_amount: view.tax_amount,
        }
    }
}

impl From<WalletTokenTaxAuditEventView> for WalletTokenTaxAuditEventResponse {
    fn from(view: WalletTokenTaxAuditEventView) -> Self {
        Self {
            actor_user_id: view.actor_user_id,
            created_at: view.created_at,
            id: view.id,
            new_tax_amount: view.new_tax_amount,
            operation: view.operation.as_str().to_string(),
            previous_tax_amount: view.previous_tax_amount,
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
