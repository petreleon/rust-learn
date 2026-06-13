use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::course_service::{
    discover_organization_courses, OrganizationCourseListError, OrganizationCourseListQuery,
};
use crate::utils::request_auth::authenticated_user;

use super::dto::OrganizationCourseListParams;

pub(super) async fn get_organization_courses(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<OrganizationCourseListParams>,
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

    let list_query = OrganizationCourseListQuery::new(
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match discover_organization_courses(&mut conn, requester.user_id, organization_id, list_query)
        .await
    {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(OrganizationCourseListError::PermissionDenied(_)) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization courses"),
        Err(OrganizationCourseListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationCourseListError::Database(error)) => {
            log::error!(
                "event=organization_courses_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization courses")
        }
    }
}
