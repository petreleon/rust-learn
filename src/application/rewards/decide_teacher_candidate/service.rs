use futures::future::BoxFuture;

use crate::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecisionCommand, TeacherRewardCandidateDecisionError,
    TeacherRewardCandidateDecisionOutput,
};

pub trait TeacherRewardCandidateDecisionUseCase: Send + Sync {
    fn decide_teacher_reward_candidate(
        &self,
        actor_user_id: i32,
        course_id: i32,
        candidate_id: i64,
        command: TeacherRewardCandidateDecisionCommand,
    ) -> BoxFuture<
        '_,
        Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError>,
    >;
}
