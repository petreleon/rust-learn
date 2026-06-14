use crate::application::rewards::reconcile_candidate::RewardReconciliationError;
use crate::domain::rewards::candidate::lifecycle;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn ensure_candidate_reconcilable(
    candidate: &RewardCandidate,
) -> Result<(), RewardReconciliationError> {
    match RewardCandidateStatus::parse(&candidate.status) {
        Ok(status) if lifecycle::can_reconcile(status) => Ok(()),
        _ => Err(RewardReconciliationError::InvalidStatus(
            "reward candidate has no confirmed state to reconcile".to_string(),
        )),
    }
}

pub(super) fn should_create_reconciliation_wallet_credit(candidate: &RewardCandidate) -> bool {
    RewardCandidateStatus::parse(&candidate.status)
        .map(lifecycle::should_create_reconciliation_wallet_credit)
        .unwrap_or(false)
}
