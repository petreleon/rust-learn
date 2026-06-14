use super::status::RewardCandidateStatus as Status;
use super::transition::{
    amount_decision, amount_decision_target_status, amount_decision_transition, apply_transition,
    teacher_decision, teacher_decision_target_status, teacher_decision_transition,
    TransitionAction,
};

#[test]
fn teacher_approval_starts_from_pending_teacher_approval() {
    assert_eq!(
        teacher_decision(Status::PendingTeacherApproval, true).unwrap(),
        Status::TeacherApproved
    );
    assert!(teacher_decision(Status::AmountApproved, true).is_err());
}

#[test]
fn teacher_target_transition_stays_in_domain() {
    assert_eq!(
        teacher_decision_transition(Status::PendingTeacherApproval, Status::TeacherApproved)
            .unwrap(),
        Status::TeacherApproved
    );
    assert_eq!(
        teacher_decision_transition(Status::PendingTeacherApproval, Status::TeacherRejected)
            .unwrap(),
        Status::TeacherRejected
    );
    assert!(teacher_decision_transition(Status::AmountApproved, Status::TeacherApproved).is_err());
    assert!(
        teacher_decision_transition(Status::PendingTeacherApproval, Status::AmountApproved)
            .is_err()
    );
}

#[test]
fn amount_approval_starts_from_teacher_approved() {
    assert_eq!(
        amount_decision(Status::TeacherApproved, true).unwrap(),
        Status::AmountApproved
    );
    assert!(amount_decision(Status::PendingTeacherApproval, true).is_err());
}

#[test]
fn amount_target_transition_stays_in_domain() {
    assert_eq!(
        amount_decision_transition(Status::TeacherApproved, Status::AmountApproved).unwrap(),
        Status::AmountApproved
    );
    assert_eq!(
        amount_decision_transition(Status::TeacherApproved, Status::AmountRejected).unwrap(),
        Status::AmountRejected
    );
    assert!(
        amount_decision_transition(Status::PendingTeacherApproval, Status::AmountApproved).is_err()
    );
    assert!(amount_decision_transition(Status::TeacherApproved, Status::TeacherApproved).is_err());
}

#[test]
fn normalizes_teacher_decision_target_status_aliases() {
    assert_eq!(
        teacher_decision_target_status(" approved "),
        Some(Status::TeacherApproved)
    );
    assert_eq!(
        teacher_decision_target_status("teacher_rejected"),
        Some(Status::TeacherRejected)
    );
    assert_eq!(teacher_decision_target_status("teacher-approved"), None);
}

#[test]
fn normalizes_amount_decision_target_status_aliases() {
    assert_eq!(
        amount_decision_target_status(" APPROVED "),
        Some(Status::AmountApproved)
    );
    assert_eq!(
        amount_decision_target_status("amount_rejected"),
        Some(Status::AmountRejected)
    );
    assert_eq!(amount_decision_target_status("pending"), None);
}

#[test]
fn token_confirmation_requires_token_pending() {
    assert_eq!(
        apply_transition(Status::TokenPending, TransitionAction::ConfirmToken).unwrap(),
        Status::TokenConfirmed
    );
    assert!(apply_transition(Status::AmountApproved, TransitionAction::ConfirmToken).is_err());
}

#[test]
fn wallet_credit_accepts_confirmed_off_chain_or_reconciliation_states() {
    assert_eq!(
        apply_transition(Status::TokenConfirmed, TransitionAction::CreditWallet).unwrap(),
        Status::WalletCredited
    );
    assert_eq!(
        apply_transition(Status::AmountApproved, TransitionAction::CreditWallet).unwrap(),
        Status::WalletCredited
    );
    assert_eq!(
        apply_transition(Status::NeedsReconciliation, TransitionAction::CreditWallet).unwrap(),
        Status::WalletCredited
    );
}
