use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::list_course_candidates::{
    CourseRewardCandidate, CourseRewardCandidatesError, CourseRewardCandidatesUseCase,
};
use crate::http::rewards::dto::{CourseRewardCandidateResponse, ListCourseRewardCandidatesRequest};
use crate::utils::request_auth::authenticated_user;

pub async fn list_course_reward_candidates(
    req: HttpRequest,
    path: web::Path<i32>,
    candidates: web::Data<Arc<dyn CourseRewardCandidatesUseCase>>,
    query: web::Query<ListCourseRewardCandidatesRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match candidates
        .list_course_reward_candidates(
            requester.user_id,
            path.into_inner(),
            query.into_inner().into(),
        )
        .await
    {
        Ok(candidates) => HttpResponse::Ok().json(candidate_responses(candidates)),
        Err(error) => course_reward_candidates_error_response(error),
    }
}

fn course_reward_candidates_error_response(error: CourseRewardCandidatesError) -> HttpResponse {
    match error {
        CourseRewardCandidatesError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward candidate permission")
        }
        CourseRewardCandidatesError::InvalidStatus(message) => {
            HttpResponse::Conflict().body(message)
        }
        CourseRewardCandidatesError::NotFound => {
            HttpResponse::NotFound().body("Reward candidate not found")
        }
        CourseRewardCandidatesError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        CourseRewardCandidatesError::Database(message) => {
            log::error!(
                "event=course_reward_candidates_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process reward candidate")
        }
    }
}

fn candidate_responses(
    candidates: Vec<CourseRewardCandidate>,
) -> Vec<CourseRewardCandidateResponse> {
    candidates
        .into_iter()
        .map(CourseRewardCandidateResponse::from)
        .collect()
}
