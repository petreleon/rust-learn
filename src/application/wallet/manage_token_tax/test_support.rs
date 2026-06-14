use bigdecimal::BigDecimal;
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxError, WalletTokenTaxOperation, WalletTokenTaxStore,
};

pub(crate) struct FakeWalletTokenTaxStore {
    pub can_set: bool,
    pub checked_permission: bool,
    pub loaded_operations: Vec<WalletTokenTaxOperation>,
    pub saved_tax: Option<(WalletTokenTaxOperation, BigDecimal)>,
    pub deposit_tax: BigDecimal,
    pub retire_tax: BigDecimal,
}

impl Default for FakeWalletTokenTaxStore {
    fn default() -> Self {
        Self {
            can_set: true,
            checked_permission: false,
            loaded_operations: Vec::new(),
            saved_tax: None,
            deposit_tax: BigDecimal::from(0),
            retire_tax: BigDecimal::from(0),
        }
    }
}

impl WalletTokenTaxStore for FakeWalletTokenTaxStore {
    fn can_set_token_tax(
        &mut self,
        _actor_user_id: i32,
        _operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<bool, WalletTokenTaxError>> {
        self.checked_permission = true;
        ready(Ok(self.can_set)).boxed()
    }

    fn load_token_tax(
        &mut self,
        operation: WalletTokenTaxOperation,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletTokenTaxError>> {
        self.loaded_operations.push(operation);
        let amount = match operation {
            WalletTokenTaxOperation::Deposit => self.deposit_tax.clone(),
            WalletTokenTaxOperation::Retire => self.retire_tax.clone(),
        };
        ready(Ok(amount)).boxed()
    }

    fn save_token_tax(
        &mut self,
        operation: WalletTokenTaxOperation,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<(), WalletTokenTaxError>> {
        self.saved_tax = Some((operation, amount));
        ready(Ok(())).boxed()
    }
}
