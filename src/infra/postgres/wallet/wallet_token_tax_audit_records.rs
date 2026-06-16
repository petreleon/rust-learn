use bigdecimal::BigDecimal;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::manage_token_tax::WalletTokenTaxOperation;
use crate::infra::postgres::models::wallet_token_tax_audit_event::NewWalletTokenTaxAuditEvent;
use crate::infra::postgres::schema::wallet_token_tax_audit_events;

pub(super) async fn record_token_tax_audit(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    operation: WalletTokenTaxOperation,
    previous_amount: BigDecimal,
    new_amount: BigDecimal,
) -> diesel::QueryResult<usize> {
    diesel::insert_into(wallet_token_tax_audit_events::table)
        .values(NewWalletTokenTaxAuditEvent {
            actor_user_id: Some(actor_user_id),
            operation: operation.as_str().to_string(),
            previous_tax_amount: previous_amount,
            new_tax_amount: new_amount,
        })
        .execute(conn)
        .await
}
