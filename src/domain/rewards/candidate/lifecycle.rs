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

pub fn wallet_credit_notification_target_status(
    status: RewardCandidateStatus,
    allow_reconciliation_repair: bool,
) -> Option<RewardCandidateStatus> {
    if can_create_missing_wallet_credit_notification(status, allow_reconciliation_repair) {
        Some(RewardCandidateStatus::Notified)
    } else {
        None
    }
}

pub fn requires_wallet_credit_record(status: RewardCandidateStatus) -> bool {
    use RewardCandidateStatus as Status;

    matches!(
        status,
        Status::WalletCredited | Status::Notified | Status::Completed
    )
}
