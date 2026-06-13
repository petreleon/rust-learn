use bigdecimal::BigDecimal;
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::retire_tokens::{
    WalletRetirementDraft, WalletRetirementError, WalletRetirementStore, WalletRetirementView,
};

pub(crate) struct FakeWalletRetirementStore {
    pub user_kyc_verified: bool,
    pub platform_tax: BigDecimal,
    pub checked_kyc: bool,
    pub loaded_tax: bool,
    pub retirement_draft: Option<WalletRetirementDraft>,
}

impl Default for FakeWalletRetirementStore {
    fn default() -> Self {
        Self {
            user_kyc_verified: true,
            platform_tax: BigDecimal::from(0),
            checked_kyc: false,
            loaded_tax: false,
            retirement_draft: None,
        }
    }
}

impl WalletRetirementStore for FakeWalletRetirementStore {
    fn user_kyc_verified(
        &mut self,
        _user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletRetirementError>> {
        self.checked_kyc = true;
        ready(Ok(self.user_kyc_verified)).boxed()
    }

    fn load_platform_retire_tax(
        &mut self,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletRetirementError>> {
        self.loaded_tax = true;
        ready(Ok(self.platform_tax.clone())).boxed()
    }

    fn retire_tokens(
        &mut self,
        _user_id: i32,
        draft: WalletRetirementDraft,
    ) -> BoxFuture<'_, Result<WalletRetirementView, WalletRetirementError>> {
        self.retirement_draft = Some(draft.clone());
        let wallet_delta = -(draft.amount.clone() + draft.tax_amount.clone());
        ready(Ok(WalletRetirementView {
            operation: "retire",
            wallet_id: 11,
            transaction_id: 20,
            external_transaction_id: 30,
            internal_transaction_ids: vec![40],
            amount: draft.amount.to_string(),
            tax_amount: draft.tax_amount.to_string(),
            wallet_delta: wallet_delta.to_string(),
            gas_payer: draft.gas_payer,
            ethereum_address: draft.ethereum_address,
            wallet_provider: draft.wallet_provider,
            metamask_required: draft.metamask_required,
            wallet_action: draft.wallet_action,
        }))
        .boxed()
    }
}
