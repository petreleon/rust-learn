use std::sync::Arc;

use actix_web::web;

use crate::application::learning::discover_courses::{
    CourseDiscoveryQuery, CourseDiscoveryUseCase,
};
use crate::application::learning::get_course::CourseReadUseCase;
use crate::application::learning::get_learner_course_detail::{
    LearnerCourseDetailQuery, LearnerCourseDetailUseCase,
};
use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningQuery, LearnerCourseLearningUseCase,
};
use crate::application::learning::list_learner_course_catalog::{
    LearnerCourseCatalogListUseCase, LearnerCourseCatalogQuery,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::learning::dto::{
    CourseDiscoveryResponse, CourseResponse, LearnerCourseCatalogResponse,
    LearnerCourseDetailResponse, LearnerCourseLearningResponse,
};

use super::dto::{CourseDiscoveryParams, LearnerCourseCatalogParams};
use super::errors::{course_discovery_error, course_read_error, learner_course_read_error};

pub(super) async fn list_learner_course_catalog(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn LearnerCourseCatalogListUseCase>>,
    query: web::Query<LearnerCourseCatalogParams>,
) -> Result<web::Json<LearnerCourseCatalogResponse>, ApiError> {
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

    use_case
        .list_learner_course_catalog(catalog_query)
        .await
        .map(LearnerCourseCatalogResponse::from)
        .map(web::Json)
        .map_err(learner_course_read_error)
}

pub(super) async fn get_learner_course_learning_route(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn LearnerCourseLearningUseCase>>,
) -> Result<web::Json<LearnerCourseLearningResponse>, ApiError> {
    let query = LearnerCourseLearningQuery::new(requester.user_id(), path.into_inner());
    use_case
        .get_learner_course_learning(query)
        .await
        .map(LearnerCourseLearningResponse::from)
        .map(web::Json)
        .map_err(learner_course_read_error)
}

pub(super) async fn get_learner_course_catalog_detail(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn LearnerCourseDetailUseCase>>,
) -> Result<web::Json<LearnerCourseDetailResponse>, ApiError> {
    let query = LearnerCourseDetailQuery::new(requester.user_id(), path.into_inner());
    use_case
        .get_learner_course_detail(query)
        .await
        .map(LearnerCourseDetailResponse::from)
        .map(web::Json)
        .map_err(learner_course_read_error)
}

pub(super) async fn list_courses(
    use_case: web::Data<Arc<dyn CourseDiscoveryUseCase>>,
    query: web::Query<CourseDiscoveryParams>,
) -> Result<web::Json<CourseDiscoveryResponse>, ApiError> {
    let discovery = CourseDiscoveryQuery::new(
        query.search.clone(),
        query.organization_id,
        query.limit,
        query.offset,
    );
    let log_query = discovery.clone();

    use_case
        .discover_courses(discovery)
        .await
        .map(CourseDiscoveryResponse::from)
        .map(web::Json)
        .map_err(|error| course_discovery_error(&log_query, error))
}

pub(super) async fn get_course(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseReadUseCase>>,
) -> Result<web::Json<CourseResponse>, ApiError> {
    let course_id = path.into_inner();

    use_case
        .get_course(course_id)
        .await
        .map(CourseResponse::from)
        .map(web::Json)
        .map_err(|error| course_read_error(course_id, error))
}
