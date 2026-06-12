use crate::db;
use crate::services::kyc_service::{self, KycDecisionRequest, KycError, SubmitKycRequest};
use crate::utils::request_auth::authenticated_user;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

fn service_error_response(error: KycError) -> HttpResponse {
    match error {
        KycError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        KycError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        KycError::InvalidTransition(message) => HttpResponse::Conflict().body(message),
        KycError::NotFound => HttpResponse::NotFound().body("KYC submission not found"),
        KycError::Database(message) => {
            log::error!("event=kyc_api_failed reason=database error={}", message);
            HttpResponse::InternalServerError().body("Failed to process KYC request")
        }
    }
}

async fn get_my_kyc(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    match kyc_service::get_my_status(&mut conn, requester.user_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(error) => service_error_response(error),
    }
}

async fn submit_my_kyc(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<SubmitKycRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    match kyc_service::submit_my_kyc(&mut conn, requester.user_id, body.into_inner()).await {
        Ok(status) => HttpResponse::Created().json(status),
        Err(error) => service_error_response(error),
    }
}

async fn list_review_queue(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    match kyc_service::list_review_queue(&mut conn, requester.user_id).await {
        Ok(queue) => HttpResponse::Ok().json(queue),
        Err(error) => service_error_response(error),
    }
}

async fn decide_kyc(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    path: web::Path<i64>,
    body: web::Json<KycDecisionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    match kyc_service::decide_submission(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner(),
    )
    .await
    {
        Ok(submission) => HttpResponse::Ok().json(submission),
        Err(error) => service_error_response(error),
    }
}

pub fn kyc_scope() -> actix_web::Scope {
    web::scope("/kyc")
        .service(
            web::resource("/me")
                .route(web::get().to(get_my_kyc))
                .route(web::post().to(submit_my_kyc)),
        )
        .service(web::resource("/review").route(web::get().to(list_review_queue)))
        .service(web::resource("/review/{id}").route(web::put().to(decide_kyc)))
}
