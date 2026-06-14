use crate::application::wallet::retire_tokens::WalletRetirementError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletRetirementGasPayer {
    User,
    Platform,
}

impl WalletRetirementGasPayer {
    pub fn parse(value: &str) -> Result<Self, WalletRetirementError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "user" => Ok(Self::User),
            "platform" => Ok(Self::Platform),
            _ => Err(WalletRetirementError::InvalidInput(
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
        match self {
            Self::User => "metamask",
            Self::Platform => "platform",
        }
    }

    pub fn metamask_required(self) -> bool {
        matches!(self, Self::User)
    }

    pub fn wallet_action(self) -> &'static str {
        match self {
            Self::User => "metamask_presigned_transfer",
            Self::Platform => "platform_transfer",
        }
    }
}
