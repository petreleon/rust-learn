use super::status::RewardCandidateStatus;

pub fn can_reconcile(status: RewardCandidateStatus) -> bool {
    use RewardCandidateStatus as Status;

    matches!(
        status,
        Status::AmountApproved
            | Status::TokenConfirmed
            | Status::WalletCredited
            | Status::Notified
            | Status::Completed
            | Status::NeedsReconciliation
    )
}

pub fn should_create_reconciliation_wallet_credit(status: RewardCandidateStatus) -> bool {
    use RewardCandidateStatus as Status;

    matches!(
        status,
        Status::AmountApproved | Status::TokenConfirmed | Status::NeedsReconciliation
    )
}

pub fn can_inspect_wallet_credit_notification(
    status: RewardCandidateStatus,
    allow_reconciliation_repair: bool,
) -> bool {
    use RewardCandidateStatus as Status;

    matches!(
        status,
        Status::WalletCredited | Status::Notified | Status::Completed | Status::NeedsReconciliation
    ) || (allow_reconciliation_repair && should_create_reconciliation_wallet_credit(status))
}

pub fn can_create_missing_wallet_credit_notification(
    status: RewardCandidateStatus,
    allow_reconciliation_repair: bool,
) -> bool {
    status == RewardCandidateStatus::WalletCredited
        || (allow_reconciliation_repair && should_create_reconciliation_wallet_credit(status))
}

#[cfg(test)]
mod tests {
    use super::{
        can_create_missing_wallet_credit_notification, can_inspect_wallet_credit_notification,
        can_reconcile, should_create_reconciliation_wallet_credit,
    };
    use crate::domain::rewards::candidate::status::RewardCandidateStatus as Status;

    #[test]
    fn reconciliation_accepts_confirmed_or_later_candidate_states() {
        for status in [
            Status::AmountApproved,
            Status::TokenConfirmed,
            Status::WalletCredited,
            Status::Notified,
            Status::Completed,
            Status::NeedsReconciliation,
        ] {
            assert!(can_reconcile(status));
        }

        assert!(!can_reconcile(Status::TeacherApproved));
        assert!(!can_reconcile(Status::Failed));
    }

    #[test]
    fn reconciliation_wallet_credit_is_created_only_for_pre_credit_states() {
        assert!(should_create_reconciliation_wallet_credit(
            Status::AmountApproved
        ));
        assert!(should_create_reconciliation_wallet_credit(
            Status::TokenConfirmed
        ));
        assert!(should_create_reconciliation_wallet_credit(
            Status::NeedsReconciliation
        ));
        assert!(!should_create_reconciliation_wallet_credit(
            Status::WalletCredited
        ));
        assert!(!should_create_reconciliation_wallet_credit(
            Status::Notified
        ));
    }

    #[test]
    fn wallet_credit_notification_inspection_supports_credited_and_repair_states() {
        assert!(can_inspect_wallet_credit_notification(
            Status::WalletCredited,
            false
        ));
        assert!(can_inspect_wallet_credit_notification(
            Status::Notified,
            false
        ));
        assert!(can_inspect_wallet_credit_notification(
            Status::Completed,
            false
        ));
        assert!(can_inspect_wallet_credit_notification(
            Status::NeedsReconciliation,
            false
        ));
        assert!(!can_inspect_wallet_credit_notification(
            Status::TokenConfirmed,
            false
        ));
        assert!(can_inspect_wallet_credit_notification(
            Status::AmountApproved,
            true
        ));
        assert!(can_inspect_wallet_credit_notification(
            Status::TokenConfirmed,
            true
        ));
    }

    #[test]
    fn missing_wallet_credit_notification_is_created_for_wallet_credit_or_repair() {
        assert!(can_create_missing_wallet_credit_notification(
            Status::WalletCredited,
            false
        ));
        assert!(!can_create_missing_wallet_credit_notification(
            Status::Notified,
            false
        ));
        assert!(!can_create_missing_wallet_credit_notification(
            Status::TokenConfirmed,
            false
        ));
        assert!(can_create_missing_wallet_credit_notification(
            Status::AmountApproved,
            true
        ));
        assert!(can_create_missing_wallet_credit_notification(
            Status::TokenConfirmed,
            true
        ));
        assert!(can_create_missing_wallet_credit_notification(
            Status::NeedsReconciliation,
            true
        ));
    }
}
