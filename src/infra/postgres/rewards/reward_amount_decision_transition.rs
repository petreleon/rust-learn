use crate::application::rewards::decide_amount::RewardAmountDecisionError;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::candidate::transition;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn ensure_amount_transition(
    current_status: &str,
    target_status: RewardCandidateStatus,
) -> Result<(), RewardAmountDecisionError> {
    let current_status = RewardCandidateStatus::parse(current_status).map_err(|_| {
        RewardAmountDecisionError::InvalidStatus(
            "reward amount can be decided only after teacher approval".to_string(),
        )
    })?;
    transition::amount_decision_transition(current_status, target_status).map_err(|_| {
        RewardAmountDecisionError::InvalidStatus(
            "reward amount can be decided only after teacher approval".to_string(),
        )
    })?;
    Ok(())
}

pub(super) fn candidate_teacher_user_ids(candidate: &RewardCandidate) -> Vec<i32> {
    let mut user_ids = vec![candidate.submitter_user_id];
    if let Some(teacher_approver_user_id) = candidate.teacher_approver_user_id {
        if !user_ids.contains(&teacher_approver_user_id) {
            user_ids.push(teacher_approver_user_id);
        }
    }
    user_ids
}
