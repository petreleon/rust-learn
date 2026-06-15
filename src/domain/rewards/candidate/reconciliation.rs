use super::status::RewardCandidateStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewardReconciliationFacts<'a> {
    pub candidate_status: &'a str,
    pub has_credit_record: bool,
    pub has_notification_record: bool,
    pub has_payout_record: bool,
}

pub fn reward_reconciliation_status(facts: RewardReconciliationFacts<'_>) -> &'static str {
    match RewardCandidateStatus::parse(facts.candidate_status).ok() {
        Some(RewardCandidateStatus::NeedsReconciliation) => "needs_reconciliation",
        Some(RewardCandidateStatus::TokenConfirmed) if !facts.has_payout_record => {
            "needs_payout_record"
        }
        Some(RewardCandidateStatus::TokenConfirmed) if !facts.has_credit_record => {
            "needs_wallet_credit"
        }
        Some(RewardCandidateStatus::WalletCredited) if !facts.has_credit_record => {
            "needs_wallet_credit_record"
        }
        Some(RewardCandidateStatus::WalletCredited) => "needs_notification",
        Some(RewardCandidateStatus::Notified | RewardCandidateStatus::Completed)
            if !facts.has_notification_record =>
        {
            "needs_notification_record"
        }
        Some(RewardCandidateStatus::Notified | RewardCandidateStatus::Completed) => "reconciled",
        Some(RewardCandidateStatus::AmountApproved | RewardCandidateStatus::TokenPending) => {
            "pending_execution"
        }
        Some(
            RewardCandidateStatus::AmountRejected
            | RewardCandidateStatus::TeacherRejected
            | RewardCandidateStatus::Failed,
        ) => "closed_without_payout",
        Some(
            RewardCandidateStatus::PendingTeacherApproval | RewardCandidateStatus::TeacherApproved,
        ) => "pending_decision",
        _ if facts.has_credit_record => "reconciled",
        _ => "pending",
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
            reward_reconciliation_status(status("needs_reconciliation")),
            "needs_reconciliation"
        );
        assert_eq!(
            reward_reconciliation_status(status("token_confirmed")),
            "needs_payout_record"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_payout_record: true,
                ..status("token_confirmed")
            }),
            "needs_wallet_credit"
        );
        assert_eq!(
            reward_reconciliation_status(status("wallet_credited")),
            "needs_wallet_credit_record"
        );
    }

    #[test]
    fn reports_notification_gaps_and_reconciled_states() {
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                ..status("wallet_credited")
            }),
            "needs_notification"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                ..status("notified")
            }),
            "needs_notification_record"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                has_notification_record: true,
                ..status("completed")
            }),
            "reconciled"
        );
    }

    #[test]
    fn classifies_pending_closed_and_unknown_statuses() {
        assert_eq!(
            reward_reconciliation_status(status("amount_approved")),
            "pending_execution"
        );
        assert_eq!(
            reward_reconciliation_status(status("teacher_rejected")),
            "closed_without_payout"
        );
        assert_eq!(
            reward_reconciliation_status(status("teacher_approved")),
            "pending_decision"
        );
        assert_eq!(
            reward_reconciliation_status(status("custom_status")),
            "pending"
        );
        assert_eq!(
            reward_reconciliation_status(RewardReconciliationFacts {
                has_credit_record: true,
                ..status("custom_status")
            }),
            "reconciled"
        );
    }
}
