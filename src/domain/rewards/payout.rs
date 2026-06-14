use std::fmt;

use crate::domain::rewards::policy::RewardPaymentStrategy;

pub const REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER: &str = "presigner_transfer";
pub const REWARD_PAYOUT_METHOD_TREASURY_TRANSFER: &str = "treasury_transfer";
pub const REWARD_PAYOUT_METHOD_MINT: &str = "mint";
pub const REWARD_PAYOUT_METHOD_OFF_CHAIN: &str = "off_chain";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardPayoutMethod {
    PresignerTransfer,
    TreasuryTransfer,
    Mint,
    OffChain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayoutMethodParseError {
    value: String,
}

impl RewardPayoutMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PresignerTransfer => REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER,
            Self::TreasuryTransfer => REWARD_PAYOUT_METHOD_TREASURY_TRANSFER,
            Self::Mint => REWARD_PAYOUT_METHOD_MINT,
            Self::OffChain => REWARD_PAYOUT_METHOD_OFF_CHAIN,
        }
    }

    pub fn parse(value: &str) -> Result<Self, PayoutMethodParseError> {
        match value {
            REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER => Ok(Self::PresignerTransfer),
            REWARD_PAYOUT_METHOD_TREASURY_TRANSFER => Ok(Self::TreasuryTransfer),
            REWARD_PAYOUT_METHOD_MINT => Ok(Self::Mint),
            REWARD_PAYOUT_METHOD_OFF_CHAIN => Ok(Self::OffChain),
            other => Err(PayoutMethodParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn for_payment_strategy(
        strategy: RewardPaymentStrategy,
        has_presigner_contract: bool,
    ) -> Self {
        match strategy {
            RewardPaymentStrategy::TreasuryTransfer if has_presigner_contract => {
                Self::PresignerTransfer
            }
            RewardPaymentStrategy::TreasuryTransfer => Self::TreasuryTransfer,
            RewardPaymentStrategy::Mint => Self::Mint,
            RewardPaymentStrategy::OffChain => Self::OffChain,
        }
    }

    pub fn requires_token_confirmation(self) -> bool {
        self != Self::OffChain
    }
}

impl fmt::Display for RewardPayoutMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for PayoutMethodParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown reward payout method '{}'", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RewardPayoutMethod, REWARD_PAYOUT_METHOD_MINT, REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER,
    };
    use crate::domain::rewards::policy::RewardPaymentStrategy;

    #[test]
    fn exposes_stable_payout_method_keys() {
        assert_eq!(
            RewardPayoutMethod::PresignerTransfer.as_str(),
            REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER
        );
        assert_eq!(RewardPayoutMethod::Mint.as_str(), REWARD_PAYOUT_METHOD_MINT);
    }

    #[test]
    fn parses_known_payout_method() {
        assert_eq!(
            RewardPayoutMethod::parse("mint").unwrap(),
            RewardPayoutMethod::Mint
        );
    }

    #[test]
    fn rejects_unknown_payout_method() {
        assert!(RewardPayoutMethod::parse("wire").is_err());
    }

    #[test]
    fn selects_payout_method_from_payment_strategy() {
        assert_eq!(
            RewardPayoutMethod::for_payment_strategy(RewardPaymentStrategy::TreasuryTransfer, true),
            RewardPayoutMethod::PresignerTransfer
        );
        assert_eq!(
            RewardPayoutMethod::for_payment_strategy(RewardPaymentStrategy::OffChain, false),
            RewardPayoutMethod::OffChain
        );
    }

    #[test]
    fn off_chain_payout_skips_token_confirmation() {
        assert!(!RewardPayoutMethod::OffChain.requires_token_confirmation());
        assert!(RewardPayoutMethod::Mint.requires_token_confirmation());
    }
}
