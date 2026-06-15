use crate::application::rewards::list_reward_history::StudentRewardHistoryError;
use crate::http::errors::ApiError;

pub(in crate::http::rewards) fn reward_history_error(error: StudentRewardHistoryError) -> ApiError {
    match error {
        StudentRewardHistoryError::InvalidInput(message) => super::invalid_input(message),
        StudentRewardHistoryError::Connection(_) => super::db_connection_failed(),
        StudentRewardHistoryError::Database(message) => super::logged_internal(
            "student_reward_history_api_failed",
            "Failed to load reward history",
            message,
        ),
    }
}
