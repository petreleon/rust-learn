use crate::db;
use crate::services::reward_candidate_service::{
    self, ListRewardCandidatesRequest, RewardAmountDecisionRequest, RewardCandidateError,
    SubmitRewardCandidateRequest, TeacherRewardCandidateDecisionRequest,
};
use crate::services::reward_history_service::{
    self, StudentRewardHistoryError, StudentRewardHistoryRequest,
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

fn reward_history_error_response(error: StudentRewardHistoryError) -> HttpResponse {
    match error {
        StudentRewardHistoryError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        StudentRewardHistoryError::Database(message) => {
            log::error!("event=student_reward_history_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load reward history")
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

async fn decide_reward_candidate_by_teacher(
    req: HttpRequest,
    path: web::Path<(i32, i64)>,
    pool: web::Data<db::DbPool>,
    body: web::Json<TeacherRewardCandidateDecisionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
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

async fn list_my_reward_history(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<StudentRewardHistoryRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_history_service::list_student_reward_history(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(error) => reward_history_error_response(error),
    }
}

async fn list_course_reward_candidates(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListRewardCandidatesRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
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

pub fn configure_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/reward-candidates/me/history").route(web::get().to(list_my_reward_history)),
    )
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
    );
}

pub fn configure_course_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{course_id}/reward-candidates")
            .route(web::post().to(submit_course_reward_candidate))
            .route(web::get().to(list_course_reward_candidates)),
    )
    .service(
        web::resource("/{course_id}/reward-candidates/{candidate_id}/teacher-decision")
            .route(web::put().to(decide_reward_candidate_by_teacher)),
    );
}

pub fn configure_organization_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{organization_id}/courses/{course_id}/reward-candidates")
            .route(web::post().to(submit_organization_reward_candidate)),
    );
}

pub fn reward_candidate_scope() -> actix_web::Scope {
    web::scope("").configure(configure_reward_candidate_routes)
}
