use super::lifecycle::{
    can_create_missing_wallet_credit_notification, can_inspect_wallet_credit_notification,
    can_reconcile, requires_wallet_credit_payout_evidence, requires_wallet_credit_record,
    should_create_reconciliation_wallet_credit, wallet_credit_notification_target_status,
};
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

#[test]
fn wallet_credit_notification_target_status_is_named_in_domain() {
    assert_eq!(
        wallet_credit_notification_target_status(Status::WalletCredited, false),
        Some(Status::Notified)
    );
    assert_eq!(
        wallet_credit_notification_target_status(Status::TokenConfirmed, false),
        None
    );
    assert_eq!(
        wallet_credit_notification_target_status(Status::AmountApproved, true),
        Some(Status::Notified)
    );
    assert_eq!(
        wallet_credit_notification_target_status(Status::NeedsReconciliation, true),
        Some(Status::Notified)
    );
}

#[test]
fn wallet_credit_record_is_required_after_crediting() {
    assert!(requires_wallet_credit_record(Status::WalletCredited));
    assert!(requires_wallet_credit_record(Status::Notified));
    assert!(requires_wallet_credit_record(Status::Completed));
    assert!(!requires_wallet_credit_record(Status::TokenConfirmed));
    assert!(!requires_wallet_credit_record(Status::NeedsReconciliation));
}

#[test]
fn wallet_credit_payout_evidence_is_required_for_reconciliation_state() {
    assert!(requires_wallet_credit_payout_evidence(
        Status::NeedsReconciliation
    ));
    assert!(!requires_wallet_credit_payout_evidence(
        Status::AmountApproved
    ));
    assert!(!requires_wallet_credit_payout_evidence(
        Status::TokenConfirmed
    ));
    assert!(!requires_wallet_credit_payout_evidence(
        Status::WalletCredited
    ));
}
