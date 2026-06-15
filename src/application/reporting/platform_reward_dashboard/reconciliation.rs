use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RewardReconciliationMismatchFacts {
    pub status: RewardCandidateStatus,
    pub has_payout_record: bool,
    pub has_wallet_credit_record: bool,
    pub has_notification_record: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardReconciliationMismatchType {
    NeedsReconciliation,
    NeedsPayoutRecord,
    NeedsWalletCredit,
    NeedsWalletCreditRecord,
    NeedsNotification,
    NeedsNotificationRecord,
}

impl RewardReconciliationMismatchType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NeedsReconciliation => "needs_reconciliation",
            Self::NeedsPayoutRecord => "needs_payout_record",
            Self::NeedsWalletCredit => "needs_wallet_credit",
            Self::NeedsWalletCreditRecord => "needs_wallet_credit_record",
            Self::NeedsNotification => "needs_notification",
            Self::NeedsNotificationRecord => "needs_notification_record",
        }
    }
}

pub(crate) fn reconciliation_mismatch_candidate_statuses() -> [RewardCandidateStatus; 5] {
    [
        RewardCandidateStatus::TokenConfirmed,
        RewardCandidateStatus::WalletCredited,
        RewardCandidateStatus::Notified,
        RewardCandidateStatus::Completed,
        RewardCandidateStatus::NeedsReconciliation,
    ]
}

pub(crate) fn classify_reward_reconciliation_mismatch(
    facts: RewardReconciliationMismatchFacts,
) -> Option<RewardReconciliationMismatchType> {
    match facts.status {
        RewardCandidateStatus::NeedsReconciliation => {
            Some(RewardReconciliationMismatchType::NeedsReconciliation)
        }
        RewardCandidateStatus::TokenConfirmed if !facts.has_payout_record => {
            Some(RewardReconciliationMismatchType::NeedsPayoutRecord)
        }
        RewardCandidateStatus::TokenConfirmed if !facts.has_wallet_credit_record => {
            Some(RewardReconciliationMismatchType::NeedsWalletCredit)
        }
        RewardCandidateStatus::WalletCredited if !facts.has_wallet_credit_record => {
            Some(RewardReconciliationMismatchType::NeedsWalletCreditRecord)
        }
        RewardCandidateStatus::WalletCredited if !facts.has_notification_record => {
            Some(RewardReconciliationMismatchType::NeedsNotification)
        }
        RewardCandidateStatus::Notified | RewardCandidateStatus::Completed
            if !facts.has_notification_record =>
        {
            Some(RewardReconciliationMismatchType::NeedsNotificationRecord)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus as Status;

    fn facts(status: Status) -> RewardReconciliationMismatchFacts {
        RewardReconciliationMismatchFacts {
            status,
            has_payout_record: false,
            has_wallet_credit_record: false,
            has_notification_record: false,
        }
    }

    #[test]
    fn exposes_candidate_statuses_scanned_for_mismatches() {
        assert_eq!(
            reconciliation_mismatch_candidate_statuses(),
            [
                Status::TokenConfirmed,
                Status::WalletCredited,
                Status::Notified,
                Status::Completed,
                Status::NeedsReconciliation
            ]
        );
    }

    #[test]
    fn classifies_token_confirmation_gaps_in_order() {
        assert_eq!(
            classify_reward_reconciliation_mismatch(facts(Status::TokenConfirmed))
                .map(RewardReconciliationMismatchType::as_str),
            Some("needs_payout_record")
        );
        assert_eq!(
            classify_reward_reconciliation_mismatch(RewardReconciliationMismatchFacts {
                has_payout_record: true,
                ..facts(Status::TokenConfirmed)
            })
            .map(RewardReconciliationMismatchType::as_str),
            Some("needs_wallet_credit")
        );
        assert_eq!(
            classify_reward_reconciliation_mismatch(RewardReconciliationMismatchFacts {
                has_payout_record: true,
                has_wallet_credit_record: true,
                ..facts(Status::TokenConfirmed)
            }),
            None
        );
    }

    #[test]
    fn classifies_wallet_credit_and_notification_gaps() {
        assert_eq!(
            classify_reward_reconciliation_mismatch(facts(Status::WalletCredited))
                .map(RewardReconciliationMismatchType::as_str),
            Some("needs_wallet_credit_record")
        );
        assert_eq!(
            classify_reward_reconciliation_mismatch(RewardReconciliationMismatchFacts {
                has_wallet_credit_record: true,
                ..facts(Status::WalletCredited)
            })
            .map(RewardReconciliationMismatchType::as_str),
            Some("needs_notification")
        );
        assert_eq!(
            classify_reward_reconciliation_mismatch(RewardReconciliationMismatchFacts {
                has_wallet_credit_record: true,
                has_notification_record: true,
                ..facts(Status::WalletCredited)
            }),
            None
        );
        assert_eq!(
            classify_reward_reconciliation_mismatch(facts(Status::Notified))
                .map(RewardReconciliationMismatchType::as_str),
            Some("needs_notification_record")
        );
    }

    #[test]
    fn marks_explicit_reconciliation_and_ignores_non_mismatch_statuses() {
        assert_eq!(
            classify_reward_reconciliation_mismatch(facts(Status::NeedsReconciliation))
                .map(RewardReconciliationMismatchType::as_str),
            Some("needs_reconciliation")
        );
        assert_eq!(
            classify_reward_reconciliation_mismatch(facts(Status::AmountApproved)),
            None
        );
    }
}
