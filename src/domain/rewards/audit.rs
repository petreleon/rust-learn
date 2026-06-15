use std::fmt;

pub type RewardAuditMetadata = serde_json::Value;

pub const REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED: &str = "candidate_submitted";
pub const REWARD_AUDIT_EVENT_TEACHER_DECISION: &str = "teacher_decision";
pub const REWARD_AUDIT_EVENT_AMOUNT_DECISION: &str = "amount_decision";
pub const REWARD_AUDIT_EVENT_TOKEN_CONFIRMED: &str = "token_confirmed";
pub const REWARD_AUDIT_EVENT_WALLET_CREDITED: &str = "wallet_credited";
pub const REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED: &str = "wallet_credit_notified";
pub const REWARD_AUDIT_EVENT_RECONCILED: &str = "reconciled";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardAuditEventType {
    CandidateSubmitted,
    TeacherDecision,
    AmountDecision,
    TokenConfirmed,
    WalletCredited,
    WalletCreditNotified,
    Reconciled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEventTypeParseError {
    value: String,
}

impl RewardAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CandidateSubmitted => REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED,
            Self::TeacherDecision => REWARD_AUDIT_EVENT_TEACHER_DECISION,
            Self::AmountDecision => REWARD_AUDIT_EVENT_AMOUNT_DECISION,
            Self::TokenConfirmed => REWARD_AUDIT_EVENT_TOKEN_CONFIRMED,
            Self::WalletCredited => REWARD_AUDIT_EVENT_WALLET_CREDITED,
            Self::WalletCreditNotified => REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED,
            Self::Reconciled => REWARD_AUDIT_EVENT_RECONCILED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, AuditEventTypeParseError> {
        match value {
            REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED => Ok(Self::CandidateSubmitted),
            REWARD_AUDIT_EVENT_TEACHER_DECISION => Ok(Self::TeacherDecision),
            REWARD_AUDIT_EVENT_AMOUNT_DECISION => Ok(Self::AmountDecision),
            REWARD_AUDIT_EVENT_TOKEN_CONFIRMED => Ok(Self::TokenConfirmed),
            REWARD_AUDIT_EVENT_WALLET_CREDITED => Ok(Self::WalletCredited),
            REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED => Ok(Self::WalletCreditNotified),
            REWARD_AUDIT_EVENT_RECONCILED => Ok(Self::Reconciled),
            other => Err(AuditEventTypeParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for RewardAuditEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for AuditEventTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward audit event type '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{RewardAuditEventType, REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED};

    #[test]
    fn exposes_stable_audit_event_keys() {
        assert_eq!(
            RewardAuditEventType::CandidateSubmitted.as_str(),
            "candidate_submitted"
        );
        assert_eq!(
            RewardAuditEventType::WalletCreditNotified.as_str(),
            REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED
        );
    }

    #[test]
    fn parses_known_audit_event_type() {
        assert_eq!(
            RewardAuditEventType::parse("teacher_decision").unwrap(),
            RewardAuditEventType::TeacherDecision
        );
    }

    #[test]
    fn rejects_unknown_audit_event_type() {
        assert!(RewardAuditEventType::parse("teacher_reviewed").is_err());
    }
}
