use std::fmt;

pub const REWARD_STATUS_PENDING_TEACHER_APPROVAL: &str = "pending_teacher_approval";
pub const REWARD_STATUS_TEACHER_APPROVED: &str = "teacher_approved";
pub const REWARD_STATUS_TEACHER_REJECTED: &str = "teacher_rejected";
pub const REWARD_STATUS_AMOUNT_APPROVED: &str = "amount_approved";
pub const REWARD_STATUS_AMOUNT_REJECTED: &str = "amount_rejected";
pub const REWARD_STATUS_ADJUSTED: &str = "adjusted";
pub const REWARD_STATUS_TOKEN_PENDING: &str = "token_pending";
pub const REWARD_STATUS_TOKEN_CONFIRMED: &str = "token_confirmed";
pub const REWARD_STATUS_WALLET_CREDITED: &str = "wallet_credited";
pub const REWARD_STATUS_NOTIFIED: &str = "notified";
pub const REWARD_STATUS_COMPLETED: &str = "completed";
pub const REWARD_STATUS_NEEDS_RECONCILIATION: &str = "needs_reconciliation";
pub const REWARD_STATUS_FAILED: &str = "failed";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardCandidateStatus {
    PendingTeacherApproval,
    TeacherApproved,
    TeacherRejected,
    AmountApproved,
    AmountRejected,
    Adjusted,
    TokenPending,
    TokenConfirmed,
    WalletCredited,
    Notified,
    Completed,
    NeedsReconciliation,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusParseError {
    value: String,
}

impl RewardCandidateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PendingTeacherApproval => REWARD_STATUS_PENDING_TEACHER_APPROVAL,
            Self::TeacherApproved => REWARD_STATUS_TEACHER_APPROVED,
            Self::TeacherRejected => REWARD_STATUS_TEACHER_REJECTED,
            Self::AmountApproved => REWARD_STATUS_AMOUNT_APPROVED,
            Self::AmountRejected => REWARD_STATUS_AMOUNT_REJECTED,
            Self::Adjusted => REWARD_STATUS_ADJUSTED,
            Self::TokenPending => REWARD_STATUS_TOKEN_PENDING,
            Self::TokenConfirmed => REWARD_STATUS_TOKEN_CONFIRMED,
            Self::WalletCredited => REWARD_STATUS_WALLET_CREDITED,
            Self::Notified => REWARD_STATUS_NOTIFIED,
            Self::Completed => REWARD_STATUS_COMPLETED,
            Self::NeedsReconciliation => REWARD_STATUS_NEEDS_RECONCILIATION,
            Self::Failed => REWARD_STATUS_FAILED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, StatusParseError> {
        match value {
            REWARD_STATUS_PENDING_TEACHER_APPROVAL => Ok(Self::PendingTeacherApproval),
            REWARD_STATUS_TEACHER_APPROVED => Ok(Self::TeacherApproved),
            REWARD_STATUS_TEACHER_REJECTED => Ok(Self::TeacherRejected),
            REWARD_STATUS_AMOUNT_APPROVED => Ok(Self::AmountApproved),
            REWARD_STATUS_AMOUNT_REJECTED => Ok(Self::AmountRejected),
            REWARD_STATUS_ADJUSTED => Ok(Self::Adjusted),
            REWARD_STATUS_TOKEN_PENDING => Ok(Self::TokenPending),
            REWARD_STATUS_TOKEN_CONFIRMED => Ok(Self::TokenConfirmed),
            REWARD_STATUS_WALLET_CREDITED => Ok(Self::WalletCredited),
            REWARD_STATUS_NOTIFIED => Ok(Self::Notified),
            REWARD_STATUS_COMPLETED => Ok(Self::Completed),
            REWARD_STATUS_NEEDS_RECONCILIATION => Ok(Self::NeedsReconciliation),
            REWARD_STATUS_FAILED => Ok(Self::Failed),
            other => Err(StatusParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<String, StatusParseError> {
        let normalized = value.trim().to_ascii_lowercase().replace(['-', ' '], "_");
        Self::parse(&normalized).map(|status| status.as_str().to_string())
    }
}

impl fmt::Display for RewardCandidateStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for StatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward candidate status '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{RewardCandidateStatus, REWARD_STATUS_WALLET_CREDITED};

    #[test]
    fn exposes_stable_status_keys() {
        assert_eq!(
            RewardCandidateStatus::WalletCredited.as_str(),
            REWARD_STATUS_WALLET_CREDITED
        );
        assert_eq!(
            RewardCandidateStatus::PendingTeacherApproval.as_str(),
            "pending_teacher_approval"
        );
    }

    #[test]
    fn parses_known_statuses() {
        assert_eq!(
            RewardCandidateStatus::parse("teacher_approved").unwrap(),
            RewardCandidateStatus::TeacherApproved
        );
    }

    #[test]
    fn rejects_unknown_status() {
        assert!(RewardCandidateStatus::parse("surprise").is_err());
    }

    #[test]
    fn normalizes_common_status_spellings() {
        assert_eq!(
            RewardCandidateStatus::normalize(" wallet-credited ").unwrap(),
            "wallet_credited"
        );
        assert_eq!(
            RewardCandidateStatus::normalize("needs reconciliation").unwrap(),
            "needs_reconciliation"
        );
    }
}
