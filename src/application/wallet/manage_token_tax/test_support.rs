use bigdecimal::BigDecimal;
use chrono::{TimeZone, Utc};
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxAuditEventView, WalletTokenTaxError, WalletTokenTaxOperation, WalletTokenTaxStore,
};

pub(crate) struct FakeWalletTokenTaxStore {
    pub can_set: bool,
    pub can_view_configuration: bool,
    pub checked_view_permission: bool,
    pub checked_permission: bool,
    pub audit_events: Vec<WalletTokenTaxAuditEventView>,
    pub loaded_operations: Vec<WalletTokenTaxOperation>,
    pub saved_tax: Option<(i32, WalletTokenTaxOperation, BigDecimal, BigDecimal)>,
    pub deposit_tax: BigDecimal,
    pub retire_tax: BigDecimal,
}

impl Default for FakeWalletTokenTaxStore {
    fn default() -> Self {
        Self {
            can_set: true,
            can_view_configuration: true,
            checked_view_permission: false,
            checked_permission: false,
            audit_events: Vec::new(),
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

    fn can_view_token_tax_configuration(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletTokenTaxError>> {
        self.checked_view_permission = true;
        ready(Ok(self.can_view_configuration)).boxed()
    }

    fn list_token_tax_audit(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<WalletTokenTaxAuditEventView>, WalletTokenTaxError>> {
        ready(Ok(self.audit_events.clone())).boxed()
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
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
        previous_amount: BigDecimal,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<(), WalletTokenTaxError>> {
        self.saved_tax = Some((
            actor_user_id,
            operation,
            previous_amount.clone(),
            amount.clone(),
        ));
        self.audit_events.insert(
            0,
            WalletTokenTaxAuditEventView {
                actor_user_id: Some(actor_user_id),
                created_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
                id: 1,
                new_tax_amount: amount.to_string(),
                operation,
                previous_tax_amount: previous_amount.to_string(),
            },
        );
        ready(Ok(())).boxed()
    }
}
