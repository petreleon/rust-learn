use std::sync::Arc;

use actix_web::web;

use crate::application::rewards::list_reward_history::{
    StudentRewardHistoryEntry, StudentRewardHistoryUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{StudentRewardHistoryEntryResponse, StudentRewardHistoryRequest};
use crate::http::rewards::errors::reward_history_error;

pub async fn list_my_reward_history(
    requester: AuthUser,
    history: web::Data<Arc<dyn StudentRewardHistoryUseCase>>,
    query: web::Query<StudentRewardHistoryRequest>,
) -> Result<web::Json<Vec<StudentRewardHistoryEntryResponse>>, ApiError> {
    history
        .list_student_reward_history(requester.user_id(), query.into_inner().into())
        .await
        .map(history_responses)
        .map(web::Json)
        .map_err(reward_history_error)
}

fn history_responses(
    entries: Vec<StudentRewardHistoryEntry>,
) -> Vec<StudentRewardHistoryEntryResponse> {
    entries
        .into_iter()
        .map(StudentRewardHistoryEntryResponse::from)
        .collect()
}
