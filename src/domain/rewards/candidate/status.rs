use std::fmt;

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
            Self::PendingTeacherApproval => "pending_teacher_approval",
            Self::TeacherApproved => "teacher_approved",
            Self::TeacherRejected => "teacher_rejected",
            Self::AmountApproved => "amount_approved",
            Self::AmountRejected => "amount_rejected",
            Self::Adjusted => "adjusted",
            Self::TokenPending => "token_pending",
            Self::TokenConfirmed => "token_confirmed",
            Self::WalletCredited => "wallet_credited",
            Self::Notified => "notified",
            Self::Completed => "completed",
            Self::NeedsReconciliation => "needs_reconciliation",
            Self::Failed => "failed",
        }
    }

    pub fn parse(value: &str) -> Result<Self, StatusParseError> {
        match value {
            "pending_teacher_approval" => Ok(Self::PendingTeacherApproval),
            "teacher_approved" => Ok(Self::TeacherApproved),
            "teacher_rejected" => Ok(Self::TeacherRejected),
            "amount_approved" => Ok(Self::AmountApproved),
            "amount_rejected" => Ok(Self::AmountRejected),
            "adjusted" => Ok(Self::Adjusted),
            "token_pending" => Ok(Self::TokenPending),
            "token_confirmed" => Ok(Self::TokenConfirmed),
            "wallet_credited" => Ok(Self::WalletCredited),
            "notified" => Ok(Self::Notified),
            "completed" => Ok(Self::Completed),
            "needs_reconciliation" => Ok(Self::NeedsReconciliation),
            "failed" => Ok(Self::Failed),
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
    use super::RewardCandidateStatus;

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
