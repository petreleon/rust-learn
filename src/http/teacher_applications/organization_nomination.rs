use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::http::teacher_applications::support::{
    notify_teacher_application_event, service_error_response,
};
use crate::services::teacher_application_service::{self, OrganizationTeacherNominationRequest};
use crate::utils::request_auth::authenticated_user;

pub async fn nominate_application(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<OrganizationTeacherNominationRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
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
        Ok(application) => {
            notify_teacher_application_event(
                &req,
                &mut conn,
                &application,
                "organization_nominated",
                None,
            )
            .await;
            HttpResponse::Created().json(application)
        }
        Err(error) => service_error_response(error),
    }
}
