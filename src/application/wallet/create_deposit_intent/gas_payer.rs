use crate::application::wallet::create_deposit_intent::WalletDepositIntentError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletDepositGasPayer {
    User,
    Platform,
}

impl WalletDepositGasPayer {
    pub fn parse(value: &str) -> Result<Self, WalletDepositIntentError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "user" => Ok(Self::User),
            "platform" => Ok(Self::Platform),
            _ => Err(WalletDepositIntentError::InvalidInput(
                "gas_payer must be 'user' or 'platform'".to_string(),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Platform => "platform",
        }
    }

    pub fn wallet_provider(self) -> &'static str {
        "metamask"
    }

    pub fn metamask_required(self) -> bool {
        true
    }

    pub fn wallet_action(self) -> &'static str {
        match self {
            Self::User => "metamask_transfer",
            Self::Platform => "metamask_permit_signature",
        }
    }
}
