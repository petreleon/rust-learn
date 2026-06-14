mod apply_wallet_token_ledger_entries;
mod configured_deposit_platform_address;
mod create_user_wallet_token_deposit_intent;
mod credit_observed_wallet_deposit;
mod ensure_user_kyc_verified;
mod link_organization_wallet;
mod support;
mod validate_positive_amount;
mod wallet_token_helpers;

pub use credit_observed_wallet_deposit::credit_observed_wallet_deposit;
pub use link_organization_wallet::{
    deposit_tokens_to_user_wallet, get_wallet_token_taxes, link_organization_wallet,
    set_wallet_token_tax_for_actor,
};
pub use support::{
    LinkedWallet, ObservedWalletDepositEvent, SetWalletTokenTaxRequest, WalletDepositCreditResult,
    WalletTokenDepositIntentResponse, WalletTokenOperation, WalletTokenTaxResponse,
    WalletTokenTaxSettingsResponse, WalletTokenTransferError, WalletTokenTransferRequest,
    TOKEN_TRANSFER_GAS_PAYER_PLATFORM, TOKEN_TRANSFER_GAS_PAYER_USER,
    TOKEN_TRANSFER_OPERATION_DEPOSIT, TOKEN_TRANSFER_OPERATION_RETIRE,
    TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK, TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM,
};
pub use wallet_token_helpers::{find_organization_wallet, find_user_wallet, link_user_wallet};

#[cfg(test)]
mod tests;
