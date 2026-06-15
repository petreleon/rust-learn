use super::status::RewardCandidateStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewardReconciliationFacts<'a> {
    pub candidate_status: &'a str,
    pub has_credit_record: bool,
    pub has_notification_record: bool,
    pub has_payout_record: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardReconciliationStatus {
    NeedsReconciliation,
    NeedsPayoutRecord,
    NeedsWalletCredit,
    NeedsWalletCreditRecord,
    NeedsNotification,
    NeedsNotificationRecord,
    Reconciled,
    PendingExecution,
    ClosedWithoutPayout,
    PendingDecision,
    Pending,
}

impl RewardReconciliationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NeedsReconciliation => "needs_reconciliation",
            Self::NeedsPayoutRecord => "needs_payout_record",
            Self::NeedsWalletCredit => "needs_wallet_credit",
            Self::NeedsWalletCreditRecord => "needs_wallet_credit_record",
            Self::NeedsNotification => "needs_notification",
            Self::NeedsNotificationRecord => "needs_notification_record",
            Self::Reconciled => "reconciled",
            Self::PendingExecution => "pending_execution",
            Self::ClosedWithoutPayout => "closed_without_payout",
            Self::PendingDecision => "pending_decision",
            Self::Pending => "pending",
        }
    }
}

pub fn reward_reconciliation_status(
    facts: RewardReconciliationFacts<'_>,
) -> RewardReconciliationStatus {
    match RewardCandidateStatus::parse(facts.candidate_status).ok() {
        Some(RewardCandidateStatus::NeedsReconciliation) => {
            RewardReconciliationStatus::NeedsReconciliation
        }
        Some(RewardCandidateStatus::TokenConfirmed) if !facts.has_payout_record => {
            RewardReconciliationStatus::NeedsPayoutRecord
        }
        Some(RewardCandidateStatus::TokenConfirmed) if !facts.has_credit_record => {
            RewardReconciliationStatus::NeedsWalletCredit
        }
        Some(RewardCandidateStatus::WalletCredited) if !facts.has_credit_record => {
            RewardReconciliationStatus::NeedsWalletCreditRecord
        }
        Some(RewardCandidateStatus::WalletCredited) => {
            RewardReconciliationStatus::NeedsNotification
        }
        Some(RewardCandidateStatus::Notified | RewardCandidateStatus::Completed)
            if !facts.has_notification_record =>
        {
            RewardReconciliationStatus::NeedsNotificationRecord
        }
        Some(RewardCandidateStatus::Notified | RewardCandidateStatus::Completed) => {
            RewardReconciliationStatus::Reconciled
        }
        Some(RewardCandidateStatus::AmountApproved | RewardCandidateStatus::TokenPending) => {
            RewardReconciliationStatus::PendingExecution
        }
        Some(
            RewardCandidateStatus::AmountRejected
            | RewardCandidateStatus::TeacherRejected
            | RewardCandidateStatus::Failed,
        ) => RewardReconciliationStatus::ClosedWithoutPayout,
        Some(
            RewardCandidateStatus::PendingTeacherApproval | RewardCandidateStatus::TeacherApproved,
        ) => RewardReconciliationStatus::PendingDecision,
        _ if facts.has_credit_record => RewardReconciliationStatus::Reconciled,
        _ => RewardReconciliationStatus::Pending,
    }
}

#[cfg(test)]
mod tests {
    use super::{reward_reconciliation_status, RewardReconciliationFacts};

    fn status(candidate_status: &'static str) -> RewardReconciliationFacts<'static> {
        RewardReconciliationFacts {
            candidate_status,
            has_credit_record: false,
            has_notification_record: false,
            has_payout_record: false,
        }
    }

    #[test]
    fn detects_missing_reward_execution_records() {
        assert_eq!(
            reward_reconciliation_status(status("needs_reconciliation")).as_str(),
            "needs_reconciliation"
        );
        assert_eq!(
            reward_reconciliation_status(status("token_confirmed")).as_str(),
            "needs_payout_record"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_payout_record: true,
                ..status("token_confirmed")
            })
            .as_str(),
            "needs_wallet_credit"
        );
        assert_eq!(
            reward_reconciliation_status(status("wallet_credited")).as_str(),
            "needs_wallet_credit_record"
        );
    }

    #[test]
    fn reports_notification_gaps_and_reconciled_states() {
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                ..status("wallet_credited")
            })
            .as_str(),
            "needs_notification"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                ..status("notified")
            })
            .as_str(),
            "needs_notification_record"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                has_notification_record: true,
                ..status("completed")
            })
            .as_str(),
            "reconciled"
        );
    }

    #[test]
    fn classifies_pending_closed_and_unknown_statuses() {
        assert_eq!(
            reward_reconciliation_status(status("amount_approved")).as_str(),
            "pending_execution"
        );
        assert_eq!(
            reward_reconciliation_status(status("teacher_rejected")).as_str(),
            "closed_without_payout"
        );
        assert_eq!(
            reward_reconciliation_status(status("teacher_approved")).as_str(),
            "pending_decision"
        );
        assert_eq!(
            reward_reconciliation_status(status("custom_status")).as_str(),
            "pending"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                ..status("custom_status")
            })
            .as_str(),
            "reconciled"
        );
    }
}
