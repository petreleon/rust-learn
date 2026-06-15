use std::fmt;

pub const WALLET_DEPOSIT_EVENT_IMPORT: &str = "import";
pub const WALLET_DEPOSIT_EVENT_TRANSFER: &str = "transfer";
pub const WALLET_DEPOSIT_STATUS_AMBIGUOUS: &str = "ambiguous";
pub const WALLET_DEPOSIT_STATUS_CREDITED: &str = "credited";
pub const WALLET_DEPOSIT_STATUS_MISMATCHED: &str = "mismatched";
pub const WALLET_DEPOSIT_STATUS_UNMATCHED: &str = "unmatched";
pub const WALLET_DEPOSIT_STATUS_PENDING: &str = "pending_chain_confirmation";
pub const WALLET_DEPOSIT_TRANSACTION_TYPE: &str = "token_deposit";
pub const WALLET_GAS_PAYER_PLATFORM: &str = "platform";
pub const WALLET_GAS_PAYER_USER: &str = "user";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletDepositStatus {
    Ambiguous,
    Credited,
    Mismatched,
    PendingChainConfirmation,
    Unmatched,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletDepositStatusParseError {
    value: String,
}

impl WalletDepositStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ambiguous => WALLET_DEPOSIT_STATUS_AMBIGUOUS,
            Self::Credited => WALLET_DEPOSIT_STATUS_CREDITED,
            Self::Mismatched => WALLET_DEPOSIT_STATUS_MISMATCHED,
            Self::PendingChainConfirmation => WALLET_DEPOSIT_STATUS_PENDING,
            Self::Unmatched => WALLET_DEPOSIT_STATUS_UNMATCHED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, WalletDepositStatusParseError> {
        match value {
            WALLET_DEPOSIT_STATUS_AMBIGUOUS => Ok(Self::Ambiguous),
            WALLET_DEPOSIT_STATUS_CREDITED => Ok(Self::Credited),
            WALLET_DEPOSIT_STATUS_MISMATCHED => Ok(Self::Mismatched),
            WALLET_DEPOSIT_STATUS_PENDING => Ok(Self::PendingChainConfirmation),
            WALLET_DEPOSIT_STATUS_UNMATCHED => Ok(Self::Unmatched),
            other => Err(WalletDepositStatusParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for WalletDepositStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for WalletDepositStatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown wallet deposit status '{}'", self.value)
    }
}

pub fn wallet_deposit_event_type_is_supported(value: &str) -> bool {
    matches!(
        value,
        WALLET_DEPOSIT_EVENT_IMPORT | WALLET_DEPOSIT_EVENT_TRANSFER
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_stable_status_keys() {
        assert_eq!(WalletDepositStatus::Credited.as_str(), "credited");
        assert_eq!(
            WalletDepositStatus::PendingChainConfirmation.as_str(),
            WALLET_DEPOSIT_STATUS_PENDING
        );
    }

    #[test]
    fn parses_known_statuses() {
        assert_eq!(
            WalletDepositStatus::parse("ambiguous").unwrap(),
            WalletDepositStatus::Ambiguous
        );
    }

    #[test]
    fn rejects_unknown_statuses() {
        assert!(WalletDepositStatus::parse("settled").is_err());
    }
}
