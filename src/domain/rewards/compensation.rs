use std::fmt;

pub const REWARD_TRANSACTION_TYPE_COMPENSATION: &str = "reward_compensation";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardCompensationTransactionType {
    Compensation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompensationTransactionTypeParseError {
    value: String,
}

impl RewardCompensationTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compensation => REWARD_TRANSACTION_TYPE_COMPENSATION,
        }
    }

    pub fn parse(value: &str) -> Result<Self, CompensationTransactionTypeParseError> {
        match value {
            REWARD_TRANSACTION_TYPE_COMPENSATION => Ok(Self::Compensation),
            other => Err(CompensationTransactionTypeParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for RewardCompensationTransactionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for CompensationTransactionTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward compensation transaction type '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{RewardCompensationTransactionType, REWARD_TRANSACTION_TYPE_COMPENSATION};

    #[test]
    fn exposes_stable_compensation_transaction_key() {
        assert_eq!(
            RewardCompensationTransactionType::Compensation.as_str(),
            REWARD_TRANSACTION_TYPE_COMPENSATION
        );
    }

    #[test]
    fn parses_known_compensation_transaction_type() {
        assert_eq!(
            RewardCompensationTransactionType::parse("reward_compensation").unwrap(),
            RewardCompensationTransactionType::Compensation
        );
    }

    #[test]
    fn rejects_unknown_compensation_transaction_type() {
        assert!(RewardCompensationTransactionType::parse("reward_wallet_credit").is_err());
    }
}
