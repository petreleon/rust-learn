use crate::db;
use crate::services::reward_candidate_service::{
    self, RewardAmountDecisionRequest, RewardCandidateError, SubmitRewardCandidateRequest,
};
use crate::utils::request_auth::authenticated_user;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

fn reward_candidate_error_response(error: RewardCandidateError) -> HttpResponse {
    match error {
        RewardCandidateError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward candidate permission")
        }
        RewardCandidateError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        RewardCandidateError::InvalidStatus(message) => HttpResponse::Conflict().body(message),
        RewardCandidateError::NotFound => {
            HttpResponse::NotFound().body("Reward candidate not found")
        }
        RewardCandidateError::Database(message) => {
            log::error!("event=reward_candidate_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward candidate")
        }
    }
}

async fn submit_course_reward_candidate(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<SubmitRewardCandidateRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::submit_course_reward_candidate(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner(),
    )
    .await
    {
        Ok(candidate) => HttpResponse::Created().json(candidate),
        Err(error) => reward_candidate_error_response(error),
    }
}

async fn submit_organization_reward_candidate(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
    body: web::Json<SubmitRewardCandidateRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (organization_id, course_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::submit_organization_reward_candidate(
        &mut conn,
        requester.user_id,
        organization_id,
        course_id,
        body.into_inner(),
    )
    .await
    {
        Ok(candidate) => HttpResponse::Created().json(candidate),
        Err(error) => reward_candidate_error_response(error),
    }
}

async fn decide_reward_amount(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
    body: web::Json<RewardAmountDecisionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::decide_reward_amount(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner(),
    )
    .await
    {
        Ok(candidate) => HttpResponse::Ok().json(candidate),
        Err(error) => reward_candidate_error_response(error),
    }
}
