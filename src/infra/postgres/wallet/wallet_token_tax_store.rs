use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::{AsyncConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};
use std::str::FromStr;

use crate::application::access_control::authorize_wallet::{
    authorize_wallet_action, WalletAuthorizationAction,
};
use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxAuditEventView, WalletTokenTaxError, WalletTokenTaxOperation, WalletTokenTaxStore,
};
use crate::infra::postgres::access_control::access_decision_store::PostgresAccessDecisionStore;
use crate::infra::postgres::models::wallet_token_tax_audit_event::WalletTokenTaxAuditEvent;
use crate::infra::postgres::operations::persistent_state::{
    get_persistent_state, set_persistent_state,
};
use crate::infra::postgres::schema::wallet_token_tax_audit_events;
use crate::infra::postgres::wallet::wallet_token_tax_audit_records::record_token_tax_audit;

const TOKEN_DEPOSIT_TAX_KEY: &str = "wallet.deposit_tax_tokens";
const TOKEN_RETIRE_TAX_KEY: &str = "wallet.retire_tax_tokens";

pub struct PostgresWalletTokenTaxStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletTokenTaxStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletTokenTaxStore for PostgresWalletTokenTaxStore<'_> {
    fn can_set_token_tax(
        &mut self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<bool, WalletTokenTaxError>> {
        authorize_set_tax(self.conn, actor_user_id, operation).boxed()
    }

    fn can_view_token_tax_configuration(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletTokenTaxError>> {
        async move {
            if authorize_set_tax(self.conn, actor_user_id, WalletTokenTaxOperation::Deposit).await?
            {
                return Ok(true);
            }
            authorize_set_tax(self.conn, actor_user_id, WalletTokenTaxOperation::Retire).await
        }
        .boxed()
    }

    fn list_token_tax_audit(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<WalletTokenTaxAuditEventView>, WalletTokenTaxError>> {
        async move {
            let events = wallet_token_tax_audit_events::table
                .order(wallet_token_tax_audit_events::created_at.desc())
                .then_order_by(wallet_token_tax_audit_events::id.desc())
                .limit(50)
                .load::<WalletTokenTaxAuditEvent>(self.conn)
                .await
                .map_err(|error| WalletTokenTaxError::TaxLoad(error.to_string()))?;
            events.into_iter().map(audit_event_view).collect()
        }
        .boxed()
    }

    fn load_token_tax(
        &mut self,
        operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletTokenTaxError>> {
        async move {
            let Some(value) = get_persistent_state(self.conn, tax_key(operation))
                .await
                .map_err(|error| WalletTokenTaxError::TaxLoad(error.to_string()))?
            else {
                return Ok(BigDecimal::from(0));
            };

            BigDecimal::from_str(value.trim()).map_err(|_| {
                WalletTokenTaxError::TaxLoad(format!(
                    "invalid stored {} token tax amount",
                    operation.as_str()
                ))
            })
        }
        .boxed()
    }

    fn save_token_tax(
        &mut self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
        previous_amount: BigDecimal,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<(), WalletTokenTaxError>> {
        async move {
            self.conn
                .transaction::<_, diesel::result::Error, _>(|conn| {
                    Box::pin(async move {
                        set_persistent_state(conn, tax_key(operation), &amount.to_string()).await?;
                        record_token_tax_audit(
                            conn,
                            actor_user_id,
                            operation,
                            previous_amount,
                            amount,
                        )
                        .await?;
                        Ok(())
                    })
                })
                .await
                .map_err(|error| WalletTokenTaxError::TaxStore(error.to_string()))
        }
        .boxed()
    }
}

async fn authorize_set_tax(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    operation: WalletTokenTaxOperation,
) -> Result<bool, WalletTokenTaxError> {
    let mut store = PostgresAccessDecisionStore::new(conn);
    authorize_wallet_action(&mut store, actor_user_id, set_tax_action(operation))
        .await
        .map_err(|error| WalletTokenTaxError::PermissionCheck(error.to_string()))
}

fn audit_event_view(
    event: WalletTokenTaxAuditEvent,
) -> Result<WalletTokenTaxAuditEventView, WalletTokenTaxError> {
    let operation = WalletTokenTaxOperation::parse(&event.operation)
        .map_err(|error| WalletTokenTaxError::TaxLoad(error.to_string()))?;
    Ok(WalletTokenTaxAuditEventView {
        actor_user_id: event.actor_user_id,
        created_at: event.created_at,
        id: event.id,
        new_tax_amount: event.new_tax_amount.to_string(),
        operation,
        previous_tax_amount: event.previous_tax_amount.to_string(),
    })
}

fn tax_key(operation: WalletTokenTaxOperation) -> &'static str {
    match operation {
        WalletTokenTaxOperation::Deposit => TOKEN_DEPOSIT_TAX_KEY,
        WalletTokenTaxOperation::Retire => TOKEN_RETIRE_TAX_KEY,
    }
}

fn set_tax_action(operation: WalletTokenTaxOperation) -> WalletAuthorizationAction {
    match operation {
        WalletTokenTaxOperation::Deposit => WalletAuthorizationAction::SetDepositTax,
        WalletTokenTaxOperation::Retire => WalletAuthorizationAction::SetRetireTax,
    }
}
