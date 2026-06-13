use crate::application::rewards::list_reward_history::{
    StudentRewardHistoryError, StudentRewardHistoryFilter, StudentRewardHistoryQuery,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

const DEFAULT_STUDENT_REWARD_HISTORY_LIMIT: i64 = 50;
const MAX_STUDENT_REWARD_HISTORY_LIMIT: i64 = 100;

pub(super) fn validated_filter(
    actor_user_id: i32,
    query: StudentRewardHistoryQuery,
) -> Result<StudentRewardHistoryFilter, StudentRewardHistoryError> {
    Ok(StudentRewardHistoryFilter {
        student_user_id: actor_user_id,
        course_id: query.course_id,
        status: query.status.map(normalized_status).transpose()?,
        limit: query
            .limit
            .unwrap_or(DEFAULT_STUDENT_REWARD_HISTORY_LIMIT)
            .clamp(1, MAX_STUDENT_REWARD_HISTORY_LIMIT),
        offset: query.offset.unwrap_or(0).max(0),
    })
}

fn normalized_status(status: String) -> Result<String, StudentRewardHistoryError> {
    RewardCandidateStatus::normalize(&status).map_err(|_| {
        StudentRewardHistoryError::InvalidInput("unsupported reward candidate status".to_string())
    })
}

#[cfg(test)]
mod tests;
