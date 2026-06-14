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

#[cfg(test)]
mod tests {
    use super::{
        transaction_type_for_token_event, RewardTokenEventType, RewardTokenTransactionType,
        REWARD_TOKEN_EVENT_TRANSFER, REWARD_TOKEN_TRANSACTION_TYPE_IMPORT,
        REWARD_TOKEN_TRANSACTION_TYPE_MINT, REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER,
    };

    #[test]
    fn exposes_stable_token_event_keys() {
        assert_eq!(
            RewardTokenEventType::Transfer.as_str(),
            REWARD_TOKEN_EVENT_TRANSFER
        );
        assert_eq!(RewardTokenEventType::Mint.as_str(), "mint");
    }

    #[test]
    fn exposes_stable_token_transaction_keys() {
        assert_eq!(
            RewardTokenTransactionType::Mint.as_str(),
            REWARD_TOKEN_TRANSACTION_TYPE_MINT
        );
        assert_eq!(
            RewardTokenTransactionType::Import.as_str(),
            REWARD_TOKEN_TRANSACTION_TYPE_IMPORT
        );
    }

    #[test]
    fn parses_known_token_event() {
        assert_eq!(
            RewardTokenEventType::parse("transfer").unwrap(),
            RewardTokenEventType::Transfer
        );
    }

    #[test]
    fn maps_token_events_to_transaction_types() {
        assert_eq!(
            RewardTokenEventType::Mint.transaction_type(),
            RewardTokenTransactionType::Mint
        );
        assert_eq!(
            RewardTokenEventType::Transfer.transaction_type(),
            RewardTokenTransactionType::Transfer
        );
        assert_eq!(
            transaction_type_for_token_event("mint").unwrap(),
            REWARD_TOKEN_TRANSACTION_TYPE_MINT
        );
        assert_eq!(
            transaction_type_for_token_event("transfer").unwrap(),
            REWARD_TOKEN_TRANSACTION_TYPE_TRANSFER
        );
        assert_eq!(
            transaction_type_for_token_event("import").unwrap(),
            REWARD_TOKEN_TRANSACTION_TYPE_IMPORT
        );
    }

    #[test]
    fn rejects_unknown_token_events() {
        assert!(transaction_type_for_token_event("").is_none());
        assert!(transaction_type_for_token_event("swap").is_none());
        assert!(transaction_type_for_token_event("burn").is_none());
        assert!(RewardTokenEventType::parse("burn").is_err());
    }
}
