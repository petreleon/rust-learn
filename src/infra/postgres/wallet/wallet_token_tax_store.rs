use bigdecimal::BigDecimal;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};
use std::str::FromStr;

use crate::application::access_control::authorize_wallet::{
    authorize_wallet_action, WalletAuthorizationAction,
};
use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxError, WalletTokenTaxOperation, WalletTokenTaxStore,
};
use crate::infra::postgres::access_control::wallet_authorization_store::PostgresWalletAuthorizationStore;
use crate::repositories::persistent_state_repository::{
    get_persistent_state, set_persistent_state,
};

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
        async move {
            let mut store = PostgresWalletAuthorizationStore::new(self.conn);
            authorize_wallet_action(&mut store, actor_user_id, set_tax_action(operation))
                .await
                .map_err(|error| WalletTokenTaxError::PermissionCheck(error.to_string()))
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
        operation: WalletTokenTaxOperation,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<(), WalletTokenTaxError>> {
        async move {
            set_persistent_state(self.conn, tax_key(operation), &amount.to_string())
                .await
                .map(|_| ())
                .map_err(|error| WalletTokenTaxError::TaxStore(error.to_string()))
        }
        .boxed()
    }
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
