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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetTransitionError {
    pub from: RewardCandidateStatus,
    pub target: RewardCandidateStatus,
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

pub fn teacher_decision_transition(
    from: RewardCandidateStatus,
    target: RewardCandidateStatus,
) -> Result<RewardCandidateStatus, TargetTransitionError> {
    use RewardCandidateStatus as Status;

    match (from, target) {
        (Status::PendingTeacherApproval, Status::TeacherApproved | Status::TeacherRejected) => {
            Ok(target)
        }
        _ => Err(TargetTransitionError { from, target }),
    }
}

pub fn amount_decision_transition(
    from: RewardCandidateStatus,
    target: RewardCandidateStatus,
) -> Result<RewardCandidateStatus, TargetTransitionError> {
    use RewardCandidateStatus as Status;

    match (from, target) {
        (Status::TeacherApproved, Status::AmountApproved | Status::AmountRejected) => Ok(target),
        _ => Err(TargetTransitionError { from, target }),
    }
}

pub fn teacher_decision_target_status(status: &str) -> Option<RewardCandidateStatus> {
    match normalize_decision_status(status).as_str() {
        "approved" | "teacher_approved" => Some(RewardCandidateStatus::TeacherApproved),
        "rejected" | "teacher_rejected" => Some(RewardCandidateStatus::TeacherRejected),
        _ => None,
    }
}

pub fn amount_decision_target_status(status: &str) -> Option<RewardCandidateStatus> {
    match normalize_decision_status(status).as_str() {
        "approved" | "amount_approved" => Some(RewardCandidateStatus::AmountApproved),
        "rejected" | "amount_rejected" => Some(RewardCandidateStatus::AmountRejected),
        _ => None,
    }
}

fn normalize_decision_status(status: &str) -> String {
    status.trim().to_ascii_lowercase()
}
