use actix_web::http::StatusCode;

use crate::http::errors::ApiError;

mod candidate;
mod fraud_block;
mod history;
mod policy;

pub(super) use candidate::{
    amount_decision_error, course_reward_candidates_error, platform_reward_candidates_error,
    reward_candidate_audit_error, reward_candidate_submission_error, teacher_decision_error,
};
pub(super) use fraud_block::reward_fraud_block_error;
pub(super) use history::reward_history_error;
pub(super) use policy::reward_policy_error;

pub(in crate::http::rewards) fn db_connection_failed() -> ApiError {
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "db_connection_failed",
        "Failed to get DB connection",
    )
}

pub(in crate::http::rewards) fn permission_denied(message: &'static str) -> ApiError {
    ApiError::new(StatusCode::FORBIDDEN, "permission_denied", message)
}

pub(in crate::http::rewards) fn invalid_input(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, "invalid_input", message)
}

pub(in crate::http::rewards) fn invalid_status(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::CONFLICT, "invalid_reward_status", message)
}

pub(in crate::http::rewards) fn candidate_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "reward_candidate_not_found",
        "Reward candidate not found",
    )
}

pub(in crate::http::rewards) fn logged_internal(
    event: &'static str,
    message: &'static str,
    error: String,
) -> ApiError {
    log::error!("event={} error={}", event, error);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "reward_request_failed",
        message,
    )
}

#[cfg(test)]
mod tests;
