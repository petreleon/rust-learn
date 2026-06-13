use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecisionError, TeacherRewardCandidateDecisionUseCase,
};
use crate::http::rewards::dto::{
    TeacherRewardCandidateDecisionRequest, TeacherRewardCandidateDecisionResponse,
};
use crate::utils::request_auth::authenticated_user;

pub async fn decide_reward_candidate_by_teacher(
    req: HttpRequest,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn TeacherRewardCandidateDecisionUseCase>>,
    body: web::Json<TeacherRewardCandidateDecisionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, candidate_id) = path.into_inner();

    match use_case
        .decide_teacher_reward_candidate(
            requester.user_id,
            course_id,
            candidate_id,
            body.into_inner().into(),
        )
        .await
    {
        Ok(candidate) => {
            HttpResponse::Ok().json(TeacherRewardCandidateDecisionResponse::from(candidate))
        }
        Err(error) => teacher_decision_error_response(error),
    }
}

fn teacher_decision_error_response(error: TeacherRewardCandidateDecisionError) -> HttpResponse {
    match error {
        TeacherRewardCandidateDecisionError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward candidate permission")
        }
        TeacherRewardCandidateDecisionError::InvalidStatus(message) => {
            HttpResponse::Conflict().body(message)
        }
        TeacherRewardCandidateDecisionError::NotFound => {
            HttpResponse::NotFound().body("Reward candidate not found")
        }
        TeacherRewardCandidateDecisionError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        TeacherRewardCandidateDecisionError::Database(message) => {
            log::error!("event=reward_candidate_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward candidate")
        }
    }
}
