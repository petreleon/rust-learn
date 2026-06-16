use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentRewardHandoffOutput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitAssessmentAttemptOutput {
    pub attempt: AssessmentAttemptOutput,
    pub score: i32,
    pub total_points: i32,
    pub percentage: i32,
    pub passed: bool,
    pub reward_handoff: AssessmentRewardHandoffOutput,
}
