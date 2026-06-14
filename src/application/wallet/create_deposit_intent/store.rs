use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::wallet::create_deposit_intent::{
    WalletDepositGasPayer, WalletDepositIntentDraft, WalletDepositIntentError,
    WalletDepositIntentView,
};

pub trait WalletDepositIntentStore {
    fn user_kyc_verified(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletDepositIntentError>>;

    fn load_platform_deposit_tax(
        &mut self,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletDepositIntentError>>;

    fn configured_deposit_platform_address(
        &mut self,
        gas_payer: WalletDepositGasPayer,
    ) -> BoxFuture<'_, Result<String, WalletDepositIntentError>>;

    fn create_deposit_intent(
        &mut self,
        user_id: i32,
        draft: WalletDepositIntentDraft,
    ) -> BoxFuture<'_, Result<WalletDepositIntentView, WalletDepositIntentError>>;
}
