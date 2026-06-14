use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::list_reward_history::{
    StudentRewardHistoryEntry, StudentRewardHistoryError, StudentRewardHistoryUseCase,
};
use crate::http::extractors::request_auth::authenticated_user;
use crate::http::rewards::dto::{StudentRewardHistoryEntryResponse, StudentRewardHistoryRequest};

pub async fn list_my_reward_history(
    req: HttpRequest,
    history: web::Data<Arc<dyn StudentRewardHistoryUseCase>>,
    query: web::Query<StudentRewardHistoryRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match history
        .list_student_reward_history(requester.user_id, query.into_inner().into())
        .await
    {
        Ok(entries) => HttpResponse::Ok().json(history_responses(entries)),
        Err(error) => reward_history_error_response(error),
    }
}

fn reward_history_error_response(error: StudentRewardHistoryError) -> HttpResponse {
    match error {
        StudentRewardHistoryError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        StudentRewardHistoryError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        StudentRewardHistoryError::Database(message) => {
            log::error!("event=student_reward_history_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load reward history")
        }
    }
}

fn history_responses(
    entries: Vec<StudentRewardHistoryEntry>,
) -> Vec<StudentRewardHistoryEntryResponse> {
    entries
        .into_iter()
        .map(StudentRewardHistoryEntryResponse::from)
        .collect()
}
