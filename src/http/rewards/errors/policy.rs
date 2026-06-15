use actix_web::http::StatusCode;

use crate::application::rewards::manage_reward_policy::RewardPolicyError;
use crate::http::errors::ApiError;

pub(in crate::http::rewards) fn reward_policy_error(error: RewardPolicyError) -> ApiError {
    match error {
        RewardPolicyError::PermissionDenied(_) => {
            super::permission_denied("User does not have reward policy permission")
        }
        RewardPolicyError::InvalidInput(message) => super::invalid_input(message),
        RewardPolicyError::NotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "reward_policy_not_found",
            "Reward policy not found",
        ),
        RewardPolicyError::Connection(_) => super::db_connection_failed(),
        RewardPolicyError::Database(message) => super::logged_internal(
            "reward_policy_api_failed",
            "Failed to process reward policy",
            message,
        ),
    }
}
