use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidateRecord, PlatformRewardCandidatesError,
};
use crate::models::reward_candidate::RewardCandidate;

impl From<RewardCandidate> for PlatformRewardCandidateRecord {
    fn from(candidate: RewardCandidate) -> Self {
        Self {
            id: candidate.id,
            course_id: candidate.course_id,
            student_user_id: candidate.student_user_id,
            submitter_user_id: candidate.submitter_user_id,
            source_scope: candidate.source_scope,
            source_organization_id: candidate.source_organization_id,
            event_type: candidate.event_type,
            status: candidate.status,
            teacher_approver_user_id: candidate.teacher_approver_user_id,
            teacher_decision_reason: candidate.teacher_decision_reason,
            approved_amount: candidate.approved_amount.map(|amount| amount.to_string()),
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        }
    }
}

pub(super) fn map_platform_reward_candidate_error(
    error: diesel::result::Error,
) -> PlatformRewardCandidatesError {
    PlatformRewardCandidatesError::Database(error.to_string())
}
