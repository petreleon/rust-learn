use actix_web::http::StatusCode;

use crate::application::rewards::manage_fraud_block::RewardFraudBlockError;
use crate::http::errors::ApiError;

pub(in crate::http::rewards) fn reward_fraud_block_error(error: RewardFraudBlockError) -> ApiError {
    match error {
        RewardFraudBlockError::PermissionDenied(_) => {
            super::permission_denied("User does not have reward fraud-block permission")
        }
        RewardFraudBlockError::InvalidInput(message) => super::invalid_input(message),
        RewardFraudBlockError::NotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "reward_fraud_block_not_found",
            "Reward fraud block not found",
        ),
        RewardFraudBlockError::Connection(_) => super::db_connection_failed(),
        RewardFraudBlockError::Database(message) => super::logged_internal(
            "reward_fraud_block_api_failed",
            "Failed to process reward fraud block",
            message,
        ),
    }
}
