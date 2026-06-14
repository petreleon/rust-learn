use super::super::apply_wallet_token_ledger_entries::{
    addresses_equal, wallet_interaction_for_transfer,
};
use super::super::support::{
    WalletTokenGasPayer, WalletTokenOperation, TOKEN_TRANSFER_ACTION_METAMASK_PERMIT_SIGNATURE,
    TOKEN_TRANSFER_ACTION_METAMASK_PRESIGNED_TRANSFER, TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER,
    TOKEN_TRANSFER_ACTION_PLATFORM_TRANSFER, TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
    TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM,
};

// ── addresses_equal ──

#[test]
fn equal_addresses_case_insensitive() {
    assert!(addresses_equal("0xABC", "0xabc"));
    assert!(addresses_equal("  0xABC  ", "0xabc"));
}

#[test]
fn different_addresses_not_equal() {
    assert!(!addresses_equal("0xABC", "0xDEF"));
}

// ── wallet_interaction_for_transfer ──

#[test]
fn user_deposit_requires_metamask() {
    let wi =
        wallet_interaction_for_transfer(WalletTokenOperation::Deposit, WalletTokenGasPayer::User);
    assert_eq!(wi.provider, TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK);
    assert!(wi.metamask_required);
    assert_eq!(wi.action, TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER);
}
#[test]
fn platform_deposit_is_permit_signature() {
    let wi = wallet_interaction_for_transfer(
        WalletTokenOperation::Deposit,
        WalletTokenGasPayer::Platform,
    );
    assert_eq!(wi.provider, TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK);
    assert!(wi.metamask_required);
    assert_eq!(wi.action, TOKEN_TRANSFER_ACTION_METAMASK_PERMIT_SIGNATURE);
}

#[test]
fn user_retire_is_presigned_transfer() {
    let wi =
        wallet_interaction_for_transfer(WalletTokenOperation::Retire, WalletTokenGasPayer::User);
    assert_eq!(wi.provider, TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK);
    assert!(wi.metamask_required);
    assert_eq!(wi.action, TOKEN_TRANSFER_ACTION_METAMASK_PRESIGNED_TRANSFER);
}

#[test]
fn platform_retire_is_platform_transfer() {
    let wi = wallet_interaction_for_transfer(
        WalletTokenOperation::Retire,
        WalletTokenGasPayer::Platform,
    );
    assert_eq!(wi.provider, TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM);
    assert!(!wi.metamask_required);
    assert_eq!(wi.action, TOKEN_TRANSFER_ACTION_PLATFORM_TRANSFER);
}
