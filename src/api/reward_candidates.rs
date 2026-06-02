use crate::db;
use crate::models::user_jwt::UserJWT;
use crate::services::reward_candidate_service::{
    self, ListRewardCandidatesRequest, RewardAmountDecisionRequest, RewardCandidateError,
    SubmitRewardCandidateRequest, TeacherRewardCandidateDecisionRequest,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

fn current_user(req: &HttpRequest) -> Result<UserJWT, HttpResponse> {
    req.extensions()
        .get::<UserJWT>()
        .cloned()
        .ok_or_else(|| HttpResponse::Unauthorized().body("Unauthorized access"))
}

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
    let requester = match current_user(&req) {
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
    let requester = match current_user(&req) {
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

async fn decide_reward_candidate_by_teacher(
    req: HttpRequest,
    path: web::Path<(i32, i64)>,
    pool: web::Data<db::DbPool>,
    body: web::Json<TeacherRewardCandidateDecisionRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, candidate_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::decide_reward_candidate_by_teacher(
        &mut conn,
        requester.user_id,
        course_id,
        candidate_id,
        body.into_inner(),
    )
    .await
    {
        Ok(candidate) => HttpResponse::Ok().json(candidate),
        Err(error) => reward_candidate_error_response(error),
    }
}

async fn decide_reward_amount(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
    body: web::Json<RewardAmountDecisionRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
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

async fn list_course_reward_candidates(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListRewardCandidatesRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::list_course_reward_candidates(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        query.into_inner(),
    )
    .await
    {
        Ok(candidates) => HttpResponse::Ok().json(candidates),
        Err(error) => reward_candidate_error_response(error),
    }
}

pub fn reward_candidate_scope() -> actix_web::Scope {
    web::scope("")
        .service(
            web::resource("/courses/{course_id}/reward-candidates")
                .route(web::post().to(submit_course_reward_candidate))
                .route(web::get().to(list_course_reward_candidates)),
        )
        .service(
            web::resource("/organizations/{organization_id}/courses/{course_id}/reward-candidates")
                .route(web::post().to(submit_organization_reward_candidate)),
        )
        .service(
            web::resource("/courses/{course_id}/reward-candidates/{candidate_id}/teacher-decision")
                .route(web::put().to(decide_reward_candidate_by_teacher)),
        )
        .service(
            web::resource("/reward-candidates/{candidate_id}/amount-decision")
                .route(web::put().to(decide_reward_amount)),
        )
}
