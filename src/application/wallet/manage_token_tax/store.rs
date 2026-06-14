use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::wallet::manage_token_tax::{WalletTokenTaxError, WalletTokenTaxOperation};

pub trait WalletTokenTaxStore {
    fn can_set_token_tax(
        &mut self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<bool, WalletTokenTaxError>>;

    fn load_token_tax(
        &mut self,
        operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletTokenTaxError>>;

    fn save_token_tax(
        &mut self,
        operation: WalletTokenTaxOperation,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<(), WalletTokenTaxError>>;
}
