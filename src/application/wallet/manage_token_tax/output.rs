use chrono::{DateTime, Utc};

use crate::application::wallet::manage_token_tax::WalletTokenTaxOperation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletTokenTaxView {
    pub operation: &'static str,
    pub tax_amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletTokenTaxSettings {
    pub deposit: WalletTokenTaxView,
    pub retire: WalletTokenTaxView,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletTokenTaxAuditEventView {
    pub id: i64,
    pub actor_user_id: Option<i32>,
    pub operation: WalletTokenTaxOperation,
    pub previous_tax_amount: String,
    pub new_tax_amount: String,
    pub created_at: DateTime<Utc>,
}
