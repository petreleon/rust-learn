use crate::application::rewards::reconcile_candidate::RewardReconciliationError;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn ensure_candidate_reconcilable(
    candidate: &RewardCandidate,
) -> Result<(), RewardReconciliationError> {
    match RewardCandidateStatus::parse(&candidate.status) {
        Ok(RewardCandidateStatus::AmountApproved)
        | Ok(RewardCandidateStatus::TokenConfirmed)
        | Ok(RewardCandidateStatus::WalletCredited)
        | Ok(RewardCandidateStatus::Notified)
        | Ok(RewardCandidateStatus::Completed)
        | Ok(RewardCandidateStatus::NeedsReconciliation) => Ok(()),
        _ => Err(RewardReconciliationError::InvalidStatus(
            "reward candidate has no confirmed state to reconcile".to_string(),
        )),
    }
}

pub(super) fn should_create_reconciliation_wallet_credit(candidate: &RewardCandidate) -> bool {
    matches!(
        RewardCandidateStatus::parse(&candidate.status),
        Ok(RewardCandidateStatus::AmountApproved)
            | Ok(RewardCandidateStatus::TokenConfirmed)
            | Ok(RewardCandidateStatus::NeedsReconciliation)
    )
}
