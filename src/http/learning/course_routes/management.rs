use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db;
use crate::db::schema::courses;
use crate::models::course::UpdateCourse;
use crate::services::course_service::{
    create_course_with_invites_for_actor, update_course_for_actor,
};
use crate::utils::request_auth::authenticated_user;

use super::dto::CreateCourseRequest;
use super::support::{course_creation_error_response, course_update_error_response};

pub(super) async fn create_course(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<CreateCourseRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = create_course_with_invites_for_actor(
        &mut conn,
        requester.user_id,
        body.title.clone(),
        body.organization_ids.clone(),
    )
    .await;

    match result {
        Ok(course) => HttpResponse::Created().json(course),
        Err(error) => course_creation_error_response(error),
    }
}

pub(super) async fn update_course(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<UpdateCourse>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result =
        update_course_for_actor(&mut conn, requester.user_id, course_id, body.into_inner()).await;

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(error) => course_update_error_response(error),
    }
}

pub(super) async fn delete_course(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(courses::table.find(course_id))
        .execute(&mut conn)
        .await;

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Course deleted")
            } else {
                HttpResponse::NotFound().body("Course not found")
            }
        }
        Err(e) => {
            log::error!(
                "event=course_delete_failed course_id={} error={}",
                course_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to delete course")
        }
    }
}
