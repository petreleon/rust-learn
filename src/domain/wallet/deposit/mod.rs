pub const WALLET_DEPOSIT_EVENT_IMPORT: &str = "import";
pub const WALLET_DEPOSIT_EVENT_TRANSFER: &str = "transfer";
pub const WALLET_DEPOSIT_STATUS_AMBIGUOUS: &str = "ambiguous";
pub const WALLET_DEPOSIT_STATUS_CREDITED: &str = "credited";
pub const WALLET_DEPOSIT_STATUS_PENDING: &str = "pending_chain_confirmation";
pub const WALLET_DEPOSIT_TRANSACTION_TYPE: &str = "token_deposit";
pub const WALLET_GAS_PAYER_PLATFORM: &str = "platform";
pub const WALLET_GAS_PAYER_USER: &str = "user";

pub fn wallet_deposit_event_type_is_supported(value: &str) -> bool {
    matches!(
        value,
        WALLET_DEPOSIT_EVENT_IMPORT | WALLET_DEPOSIT_EVENT_TRANSFER
    )
}
