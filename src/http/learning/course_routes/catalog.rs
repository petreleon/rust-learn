use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::learning::discover_courses::{
    CourseDiscoveryError, CourseDiscoveryQuery, CourseDiscoveryUseCase,
};
use crate::application::learning::get_course::{CourseReadError, CourseReadUseCase};
use crate::application::learning::get_learner_course_detail::{
    LearnerCourseDetailQuery, LearnerCourseDetailUseCase,
};
use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningQuery, LearnerCourseLearningUseCase,
};
use crate::application::learning::list_learner_course_catalog::{
    LearnerCourseCatalogListUseCase, LearnerCourseCatalogQuery,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::learning::dto::{
    CourseDiscoveryResponse, CourseResponse, LearnerCourseCatalogResponse,
    LearnerCourseDetailResponse, LearnerCourseLearningResponse,
};

use super::dto::{CourseDiscoveryParams, LearnerCourseCatalogParams};
use super::support::learner_course_read_error_response;

pub(super) async fn list_learner_course_catalog(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn LearnerCourseCatalogListUseCase>>,
    query: web::Query<LearnerCourseCatalogParams>,
) -> impl Responder {
    let catalog_query = LearnerCourseCatalogQuery::new(
        requester.user_id(),
        query.search.clone(),
        query.organization_id,
        query.lifecycle_status.clone(),
        query.enrollment_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match use_case.list_learner_course_catalog(catalog_query).await {
        Ok(catalog) => HttpResponse::Ok().json(LearnerCourseCatalogResponse::from(catalog)),
        Err(error) => learner_course_read_error_response(error),
    }
}

pub(super) async fn get_learner_course_learning_route(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn LearnerCourseLearningUseCase>>,
) -> impl Responder {
    let query = LearnerCourseLearningQuery::new(requester.user_id(), path.into_inner());
    match use_case.get_learner_course_learning(query).await {
        Ok(learning) => HttpResponse::Ok().json(LearnerCourseLearningResponse::from(learning)),
        Err(error) => learner_course_read_error_response(error),
    }
}

pub(super) async fn get_learner_course_catalog_detail(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn LearnerCourseDetailUseCase>>,
) -> impl Responder {
    let query = LearnerCourseDetailQuery::new(requester.user_id(), path.into_inner());
    match use_case.get_learner_course_detail(query).await {
        Ok(detail) => HttpResponse::Ok().json(LearnerCourseDetailResponse::from(detail)),
        Err(error) => learner_course_read_error_response(error),
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
