pub mod dto;
mod errors;
mod handlers;
mod routes;

pub use routes::{
    configure_routes, course_reward_candidates_resource,
    course_scope_reward_candidate_submission_resource,
    course_scope_teacher_reward_candidate_decision_resource,
    organization_reward_candidate_submission_resource,
    organization_scope_reward_candidate_submission_resource, platform_reward_candidates_resource,
    reward_amount_decision_resource, reward_candidate_audit_resource, reward_fraud_block_scope,
    reward_policy_scope, student_reward_history_resource,
    teacher_reward_candidate_decision_resource,
};
