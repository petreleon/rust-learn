mod create_reward_candidate;
mod decide_reward_amount;
mod ensure_active_reward_policy;
mod ensure_exact_course_permission;
mod ensure_no_active_reward_fraud_block;
mod has_active_reward_policy_fraud_block;
mod submit_course_reward_candidate;
mod submit_organization_reward_candidate;
mod support;

pub use decide_reward_amount::decide_reward_amount;
pub use submit_course_reward_candidate::submit_course_reward_candidate;
pub use submit_organization_reward_candidate::{
    decide_reward_candidate_by_teacher, submit_organization_reward_candidate,
};
pub use support::{
    RewardAmountDecisionRequest, RewardCandidateError, SubmitRewardCandidateRequest,
    TeacherRewardCandidateDecisionRequest,
};

#[cfg(test)]
mod tests;
