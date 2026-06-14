use super::lifecycle::{can_reconcile, should_create_reconciliation_wallet_credit};
use super::status::RewardCandidateStatus as Status;

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
