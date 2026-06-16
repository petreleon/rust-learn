use std::fmt;

mod audit;

pub use crate::domain::rewards::candidate::event_type::{
    RewardEventType as RewardPolicyEventType, REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT,
    REWARD_EVENT_ASSESSMENT_COMPLETION, REWARD_EVENT_COURSE_COMPLETION,
    REWARD_EVENT_MANUAL_COMPLETION,
};
pub use audit::{
    PolicyAuditEventTypeParseError, RewardPolicyAuditEventType,
    REWARD_POLICY_AUDIT_EVENT_ACTIVATED, REWARD_POLICY_AUDIT_EVENT_CREATED,
    REWARD_POLICY_AUDIT_EVENT_DEACTIVATED,
};

pub const REWARD_POLICY_SCOPE_PLATFORM: &str = "platform";
pub const REWARD_POLICY_SCOPE_ORGANIZATION: &str = "organization";
pub const REWARD_POLICY_SCOPE_COURSE: &str = "course";

pub const REWARD_PAYMENT_TREASURY_TRANSFER: &str = "treasury_transfer";
pub const REWARD_PAYMENT_MINT: &str = "mint";
pub const REWARD_PAYMENT_OFF_CHAIN: &str = "off_chain";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardPolicyScope {
    Platform,
    Organization,
    Course,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardPaymentStrategy {
    TreasuryTransfer,
    Mint,
    OffChain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyScopeParseError {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentStrategyParseError {
    value: String,
}

impl RewardPolicyScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Platform => REWARD_POLICY_SCOPE_PLATFORM,
            Self::Organization => REWARD_POLICY_SCOPE_ORGANIZATION,
            Self::Course => REWARD_POLICY_SCOPE_COURSE,
        }
    }

    pub fn parse(value: &str) -> Result<Self, PolicyScopeParseError> {
        match value {
            REWARD_POLICY_SCOPE_PLATFORM => Ok(Self::Platform),
            REWARD_POLICY_SCOPE_ORGANIZATION => Ok(Self::Organization),
            REWARD_POLICY_SCOPE_COURSE => Ok(Self::Course),
            other => Err(PolicyScopeParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, PolicyScopeParseError> {
        let normalized = value.trim().to_ascii_lowercase();
        Self::parse(&normalized)
    }
}

impl RewardPaymentStrategy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryTransfer => REWARD_PAYMENT_TREASURY_TRANSFER,
            Self::Mint => REWARD_PAYMENT_MINT,
            Self::OffChain => REWARD_PAYMENT_OFF_CHAIN,
        }
    }

    pub fn parse(value: &str) -> Result<Self, PaymentStrategyParseError> {
        match value {
            REWARD_PAYMENT_TREASURY_TRANSFER => Ok(Self::TreasuryTransfer),
            REWARD_PAYMENT_MINT => Ok(Self::Mint),
            REWARD_PAYMENT_OFF_CHAIN => Ok(Self::OffChain),
            other => Err(PaymentStrategyParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, PaymentStrategyParseError> {
        let normalized = value.trim().to_ascii_lowercase();
        Self::parse(&normalized)
    }
}

impl fmt::Display for RewardPolicyScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for RewardPaymentStrategy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for PolicyScopeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown reward policy scope '{}'", self.value)
    }
}

impl fmt::Display for PaymentStrategyParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward payment strategy '{}'",
            self.value
        )
    }
}

pub fn normalize_scope_type(scope_type: &str) -> Option<String> {
    RewardPolicyScope::normalize(scope_type)
        .map(|scope| scope.as_str().to_string())
        .ok()
}

pub fn normalize_event_type(event_type: &str) -> Option<String> {
    RewardPolicyEventType::normalize(event_type)
        .map(|event_type| event_type.as_str().to_string())
        .ok()
}

pub fn normalize_payment_strategy(payment_strategy: &str) -> Option<String> {
    RewardPaymentStrategy::normalize(payment_strategy)
        .map(|strategy| strategy.as_str().to_string())
        .ok()
}

#[cfg(test)]
mod tests;
