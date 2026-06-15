use bigdecimal::BigDecimal;
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::create_deposit_intent::{
    WalletDepositGasPayer, WalletDepositIntentDraft, WalletDepositIntentError,
    WalletDepositIntentStore, WalletDepositIntentView,
};
use crate::domain::wallet::deposit::WalletDepositStatus;

pub(crate) struct FakeWalletDepositIntentStore {
    pub user_kyc_verified: bool,
    pub platform_tax: BigDecimal,
    pub configured_platform_address: String,
    pub checked_kyc: bool,
    pub loaded_tax: bool,
    pub loaded_configuration: bool,
    pub created_draft: Option<WalletDepositIntentDraft>,
}

impl Default for FakeWalletDepositIntentStore {
    fn default() -> Self {
        Self {
            user_kyc_verified: true,
            platform_tax: BigDecimal::from(0),
            configured_platform_address: "0xplatform".to_string(),
            checked_kyc: false,
            loaded_tax: false,
            loaded_configuration: false,
            created_draft: None,
        }
    }
}

impl WalletDepositIntentStore for FakeWalletDepositIntentStore {
    fn user_kyc_verified(
        &mut self,
        _user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletDepositIntentError>> {
        self.checked_kyc = true;
        ready(Ok(self.user_kyc_verified)).boxed()
    }

    fn load_platform_deposit_tax(
        &mut self,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletDepositIntentError>> {
        self.loaded_tax = true;
        ready(Ok(self.platform_tax.clone())).boxed()
    }

    fn configured_deposit_platform_address(
        &mut self,
        _gas_payer: WalletDepositGasPayer,
    ) -> BoxFuture<'_, Result<String, WalletDepositIntentError>> {
        self.loaded_configuration = true;
        ready(Ok(self.configured_platform_address.clone())).boxed()
    }

    fn create_deposit_intent(
        &mut self,
        _user_id: i32,
        draft: WalletDepositIntentDraft,
    ) -> BoxFuture<'_, Result<WalletDepositIntentView, WalletDepositIntentError>> {
        self.created_draft = Some(draft.clone());
        ready(Ok(WalletDepositIntentView {
            operation: "deposit",
            id: 7,
            status: WalletDepositStatus::PendingChainConfirmation,
            wallet_id: 11,
            amount: draft.amount.to_string(),
            tax_amount: draft.tax_amount.to_string(),
            wallet_delta_on_confirmation: (draft.amount - draft.tax_amount).to_string(),
            gas_payer: draft.gas_payer,
            ethereum_address: draft.ethereum_address,
            platform_address: draft.platform_address,
            chain_id: draft.chain_id,
            contract_address: draft.contract_address,
            transaction_hash: draft.transaction_hash,
            log_index: draft.log_index,
            wallet_provider: draft.wallet_provider,
            metamask_required: draft.metamask_required,
            wallet_action: draft.wallet_action,
        }))
        .boxed()
    }
}
