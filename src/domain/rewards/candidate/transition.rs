use super::status::RewardCandidateStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionAction {
    TeacherApprove,
    TeacherReject,
    AmountApprove,
    AmountReject,
    MarkTokenPending,
    ConfirmToken,
    CreditWallet,
    NotifyWalletCredit,
    MarkCompleted,
    MarkNeedsReconciliation,
    MarkFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionError {
    pub from: RewardCandidateStatus,
    pub action: TransitionAction,
}

pub fn apply_transition(
    from: RewardCandidateStatus,
    action: TransitionAction,
) -> Result<RewardCandidateStatus, TransitionError> {
    use RewardCandidateStatus as Status;
    use TransitionAction as Action;

    let to = match (from, action) {
        (Status::PendingTeacherApproval, Action::TeacherApprove) => Status::TeacherApproved,
        (Status::PendingTeacherApproval, Action::TeacherReject) => Status::TeacherRejected,
        (Status::TeacherApproved, Action::AmountApprove) => Status::AmountApproved,
        (Status::TeacherApproved, Action::AmountReject) => Status::AmountRejected,
        (Status::AmountApproved, Action::MarkTokenPending) => Status::TokenPending,
        (Status::TokenPending, Action::ConfirmToken) => Status::TokenConfirmed,
        (Status::TokenConfirmed, Action::CreditWallet) => Status::WalletCredited,
        (Status::AmountApproved, Action::CreditWallet) => Status::WalletCredited,
        (Status::NeedsReconciliation, Action::CreditWallet) => Status::WalletCredited,
        (Status::WalletCredited, Action::NotifyWalletCredit) => Status::Notified,
        (Status::Notified, Action::MarkCompleted) => Status::Completed,
        (Status::AmountApproved, Action::MarkNeedsReconciliation) => Status::NeedsReconciliation,
        (Status::TokenConfirmed, Action::MarkNeedsReconciliation) => Status::NeedsReconciliation,
        (Status::WalletCredited, Action::MarkNeedsReconciliation) => Status::NeedsReconciliation,
        (_, Action::MarkFailed) => Status::Failed,
        _ => return Err(TransitionError { from, action }),
    };

    Ok(to)
}

pub fn teacher_decision(
    from: RewardCandidateStatus,
    approved: bool,
) -> Result<RewardCandidateStatus, TransitionError> {
    let action = if approved {
        TransitionAction::TeacherApprove
    } else {
        TransitionAction::TeacherReject
    };
    apply_transition(from, action)
}

pub fn amount_decision(
    from: RewardCandidateStatus,
    approved: bool,
) -> Result<RewardCandidateStatus, TransitionError> {
    let action = if approved {
        TransitionAction::AmountApprove
    } else {
        TransitionAction::AmountReject
    };
    apply_transition(from, action)
}

#[cfg(test)]
mod tests {
    use super::{amount_decision, apply_transition, teacher_decision, TransitionAction};
    use crate::domain::rewards::candidate::status::RewardCandidateStatus as Status;

    #[test]
    fn teacher_approval_starts_from_pending_teacher_approval() {
        assert_eq!(
            teacher_decision(Status::PendingTeacherApproval, true).unwrap(),
            Status::TeacherApproved
        );
        assert!(teacher_decision(Status::AmountApproved, true).is_err());
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
}
