use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::learning::discover_courses::{
    CourseDiscoveryError, CourseDiscoveryQuery, CourseDiscoveryUseCase,
};
use crate::application::learning::get_course::{CourseReadError, CourseReadUseCase};
use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningQuery, LearnerCourseLearningUseCase,
};
use crate::db;
use crate::http::learning::dto::{
    CourseDiscoveryResponse, CourseResponse, LearnerCourseLearningResponse,
};
use crate::services::course_service::{
    discover_learner_course_catalog, get_learner_course_detail, LearnerCourseCatalogQuery,
};
use crate::utils::request_auth::authenticated_user;

use super::dto::{CourseDiscoveryParams, LearnerCourseCatalogParams};
use super::support::{
    learner_course_catalog_error_response, learner_course_learning_error_response,
};

pub(super) async fn list_learner_course_catalog(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<LearnerCourseCatalogParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let catalog_query = LearnerCourseCatalogQuery::new(
        query.search.clone(),
        query.organization_id,
        query.lifecycle_status.clone(),
        query.enrollment_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match discover_learner_course_catalog(&mut conn, requester.user_id, catalog_query).await {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

pub(super) async fn get_learner_course_learning_route(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn LearnerCourseLearningUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    let query = LearnerCourseLearningQuery::new(requester.user_id, path.into_inner());
    match use_case.get_learner_course_learning(query).await {
        Ok(learning) => HttpResponse::Ok().json(LearnerCourseLearningResponse::from(learning)),
        Err(error) => learner_course_learning_error_response(error),
    }
}

pub(super) async fn get_learner_course_catalog_detail(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_learner_course_detail(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

pub(super) async fn list_courses(
    use_case: web::Data<Arc<dyn CourseDiscoveryUseCase>>,
    query: web::Query<CourseDiscoveryParams>,
) -> impl Responder {
    let discovery = CourseDiscoveryQuery::new(
        query.search.clone(),
        query.organization_id,
        query.limit,
        query.offset,
    );

    match use_case.discover_courses(discovery).await {
        Ok(course_list) => HttpResponse::Ok().json(CourseDiscoveryResponse::from(course_list)),
        Err(CourseDiscoveryError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(CourseDiscoveryError::Database(message)) => {
            log::error!(
                "event=course_list_failed search={:?} organization_id={:?} limit={:?} offset={:?} error={}",
                query.search,
                query.organization_id,
                query.limit,
                query.offset,
                message
            );
            HttpResponse::InternalServerError().body("Failed to load courses")
        }
    }
}

pub(super) async fn get_course(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseReadUseCase>>,
) -> impl Responder {
    let course_id = path.into_inner();

    match use_case.get_course(course_id).await {
        Ok(course) => HttpResponse::Ok().json(CourseResponse::from(course)),
        Err(CourseReadError::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(CourseReadError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(CourseReadError::Database(message)) => {
            log::error!(
                "event=course_fetch_failed course_id={} error={}",
                course_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch course")
        }
    }
}
