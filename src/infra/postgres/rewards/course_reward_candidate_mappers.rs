use crate::application::rewards::list_course_candidates::{
    CourseRewardCandidate, CourseRewardCandidatesError,
};
use crate::models::reward_candidate::RewardCandidate;

impl From<RewardCandidate> for CourseRewardCandidate {
    fn from(candidate: RewardCandidate) -> Self {
        Self {
            id: candidate.id,
            course_id: candidate.course_id,
            student_user_id: candidate.student_user_id,
            submitter_user_id: candidate.submitter_user_id,
            source_scope: candidate.source_scope,
            source_organization_id: candidate.source_organization_id,
            event_type: candidate.event_type,
            idempotency_key: candidate.idempotency_key,
            evidence: candidate.evidence,
            status: candidate.status,
            teacher_approver_user_id: candidate.teacher_approver_user_id,
            teacher_decision_reason: candidate.teacher_decision_reason,
            teacher_decided_at: candidate.teacher_decided_at,
            amount_reviewer_user_id: candidate.amount_reviewer_user_id,
            approved_amount: candidate.approved_amount.map(|amount| amount.to_string()),
            amount_decision_reason: candidate.amount_decision_reason,
            amount_decided_at: candidate.amount_decided_at,
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        }
    }
}

pub(super) fn map_course_reward_candidate_error(
    error: diesel::result::Error,
) -> CourseRewardCandidatesError {
    match error {
        diesel::result::Error::NotFound => CourseRewardCandidatesError::NotFound,
        other => CourseRewardCandidatesError::Database(other.to_string()),
    }
}
