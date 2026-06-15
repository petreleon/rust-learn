use std::fmt;

pub const REWARD_TOKEN_EVENT_MINT: &str = "mint";
pub const REWARD_TOKEN_EVENT_TRANSFER: &str = "transfer";
pub const REWARD_TOKEN_EVENT_IMPORT: &str = "import";

pub const REWARD_TOKEN_TRANSACTION_TYPE_MINT: &str = "token_mint";
pub const REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER: &str = "token_transfer";
pub const REWARD_TOKEN_TRANSACTION_TYPE_IMPORT: &str = "token_import";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardTokenEventType {
    Mint,
    Transfer,
    Import,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardTokenTransactionType {
    Mint,
    Transfer,
    Import,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenEventTypeParseError {
    value: String,
}

impl RewardTokenEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mint => REWARD_TOKEN_EVENT_MINT,
            Self::Transfer => REWARD_TOKEN_EVENT_TRANSFER,
            Self::Import => REWARD_TOKEN_EVENT_IMPORT,
        }
    }

    pub fn parse(value: &str) -> Result<Self, TokenEventTypeParseError> {
        match value {
            REWARD_TOKEN_EVENT_MINT => Ok(Self::Mint),
            REWARD_TOKEN_EVENT_TRANSFER => Ok(Self::Transfer),
            REWARD_TOKEN_EVENT_IMPORT => Ok(Self::Import),
            other => Err(TokenEventTypeParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, TokenEventTypeParseError> {
        Self::parse(&value.trim().to_ascii_lowercase()).map_err(|_| TokenEventTypeParseError {
            value: value.to_string(),
        })
    }

    pub fn parse_optional(value: Option<String>) -> Result<Option<Self>, TokenEventTypeParseError> {
        value.as_deref().map(Self::normalize).transpose()
    }

    pub fn transaction_type(self) -> RewardTokenTransactionType {
        match self {
            Self::Mint => RewardTokenTransactionType::Mint,
            Self::Transfer => RewardTokenTransactionType::Transfer,
            Self::Import => RewardTokenTransactionType::Import,
        }
    }
}

impl RewardTokenTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mint => REWARD_TOKEN_TRANSACTION_TYPE_MINT,
            Self::Transfer => REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER,
            Self::Import => REWARD_TOKEN_TRANSACTION_TYPE_IMPORT,
        }
    }
}

impl fmt::Display for RewardTokenEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for RewardTokenTransactionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for TokenEventTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward token event type '{}'",
            self.value
        )
    }
}

pub fn transaction_type_for_token_event(event_type: &str) -> Option<&'static str> {
    RewardTokenEventType::parse(event_type)
        .ok()
        .map(|event| event.transaction_type().as_str())
}
