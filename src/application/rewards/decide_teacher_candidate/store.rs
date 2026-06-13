use futures::future::BoxFuture;

use crate::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecisionError, TeacherRewardCandidateDecisionOutput,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherRewardCandidateDecision {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub candidate_id: i64,
    pub target_status: RewardCandidateStatus,
    pub decision_reason: Option<String>,
}

pub trait TeacherRewardCandidateDecisionStore {
    fn can_approve_student_reward_candidate(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherRewardCandidateDecisionError>>;

    fn decide_teacher_reward_candidate(
        &mut self,
        decision: TeacherRewardCandidateDecision,
    ) -> BoxFuture<
        '_,
        Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError>,
    >;
}
