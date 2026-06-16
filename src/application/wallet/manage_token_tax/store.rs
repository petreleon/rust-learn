use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxAuditEventView, WalletTokenTaxError, WalletTokenTaxOperation,
};

pub trait WalletTokenTaxStore {
    fn can_set_token_tax(
        &mut self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<bool, WalletTokenTaxError>>;

    fn can_view_token_tax_configuration(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletTokenTaxError>>;

    fn list_token_tax_audit(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<WalletTokenTaxAuditEventView>, WalletTokenTaxError>>;

    fn load_token_tax(
        &mut self,
        operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletTokenTaxError>>;

    fn save_token_tax(
        &mut self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
        previous_amount: BigDecimal,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<(), WalletTokenTaxError>>;
}
