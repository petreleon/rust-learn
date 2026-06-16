use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::wallet_token_tax_audit_events;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = wallet_token_tax_audit_events)]
pub struct WalletTokenTaxAuditEvent {
    pub id: i64,
    pub actor_user_id: Option<i32>,
    pub operation: String,
    pub previous_tax_amount: BigDecimal,
    pub new_tax_amount: BigDecimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = wallet_token_tax_audit_events)]
pub struct NewWalletTokenTaxAuditEvent {
    pub actor_user_id: Option<i32>,
    pub operation: String,
    pub previous_tax_amount: BigDecimal,
    pub new_tax_amount: BigDecimal,
}
