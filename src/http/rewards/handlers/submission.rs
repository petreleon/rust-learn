use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmissionError, RewardCandidateSubmissionUseCase,
};
use crate::http::rewards::dto::{RewardCandidateSubmissionResponse, SubmitRewardCandidateRequest};
use crate::utils::request_auth::authenticated_user;

pub async fn submit_course_reward_candidate(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn RewardCandidateSubmissionUseCase>>,
    body: web::Json<SubmitRewardCandidateRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match use_case
        .submit_course_reward_candidate(
            requester.user_id,
            path.into_inner(),
            body.into_inner().into(),
        )
        .await
    {
        Ok(candidate) => {
            HttpResponse::Created().json(RewardCandidateSubmissionResponse::from(candidate))
        }
        Err(error) => reward_candidate_submission_error_response(error),
    }
}

pub async fn submit_organization_reward_candidate(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn RewardCandidateSubmissionUseCase>>,
    body: web::Json<SubmitRewardCandidateRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (organization_id, course_id) = path.into_inner();

    match use_case
        .submit_organization_reward_candidate(
            requester.user_id,
            organization_id,
            course_id,
            body.into_inner().into(),
        )
        .await
    {
        Ok(candidate) => {
            HttpResponse::Created().json(RewardCandidateSubmissionResponse::from(candidate))
        }
        Err(error) => reward_candidate_submission_error_response(error),
    }
}

fn reward_candidate_submission_error_response(
    error: RewardCandidateSubmissionError,
) -> HttpResponse {
    match error {
        RewardCandidateSubmissionError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward candidate permission")
        }
        RewardCandidateSubmissionError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        RewardCandidateSubmissionError::InvalidStatus(message) => {
            HttpResponse::Conflict().body(message)
        }
        RewardCandidateSubmissionError::NotFound => {
            HttpResponse::NotFound().body("Reward candidate not found")
        }
        RewardCandidateSubmissionError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        RewardCandidateSubmissionError::Database(message) => {
            log::error!("event=reward_candidate_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward candidate")
        }
    }
}
