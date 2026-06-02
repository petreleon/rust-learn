use crate::db;
use crate::models::user_jwt::UserJWT;
use crate::services::teacher_application_service::{
    self, ListTeacherApplicationsRequest, OrganizationTeacherNominationRequest,
    SubmitTeacherApplicationRequest, TeacherApplicationDecisionRequest, TeacherApplicationError,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

fn current_user(req: &HttpRequest) -> Result<UserJWT, HttpResponse> {
    req.extensions()
        .get::<UserJWT>()
        .cloned()
        .ok_or_else(|| HttpResponse::Unauthorized().body("Unauthorized access"))
}

fn service_error_response(error: TeacherApplicationError) -> HttpResponse {
    match error {
        TeacherApplicationError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        TeacherApplicationError::InvalidTransition(message) => {
            HttpResponse::Conflict().body(message)
        }
        TeacherApplicationError::NotFound => {
            HttpResponse::NotFound().body("Teacher application not found")
        }
        TeacherApplicationError::Database(message) => {
            log::error!(
                "event=teacher_application_api_failed reason=database error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}

async fn submit_application(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<SubmitTeacherApplicationRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::submit_application(
        &mut conn,
        requester.user_id,
        body.into_inner(),
    )
    .await
    {
        Ok(application) => HttpResponse::Created().json(application),
        Err(error) => service_error_response(error),
    }
}

pub async fn nominate_application(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<OrganizationTeacherNominationRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::nominate_application(
        &mut conn,
        requester.user_id,
        organization_id,
        body.into_inner(),
    )
    .await
    {
        Ok(application) => HttpResponse::Created().json(application),
        Err(error) => service_error_response(error),
    }
}

async fn list_applications(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListTeacherApplicationsRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::list_applications(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(error) => service_error_response(error),
    }
}

async fn decide_application(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
    body: web::Json<TeacherApplicationDecisionRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::decide_application(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner(),
    )
    .await
    {
        Ok(application) => HttpResponse::Ok().json(application),
        Err(error) => service_error_response(error),
    }
}

async fn list_audit_events(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::list_audit_events(
        &mut conn,
        requester.user_id,
        path.into_inner(),
    )
    .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(error) => service_error_response(error),
    }
}

pub fn teacher_application_scope() -> actix_web::Scope {
    web::scope("/teacher-applications")
        .service(
            web::resource("")
                .route(web::post().to(submit_application))
                .route(web::get().to(list_applications)),
        )
        .service(web::resource("/{id}/decision").route(web::put().to(decide_application)))
        .service(web::resource("/{id}/audit").route(web::get().to(list_audit_events)))
}
