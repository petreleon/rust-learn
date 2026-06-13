use crate::config::constants::permissions::Permissions;
use crate::application::learning::assessment::AssessmentReadError;
use crate::application::learning::list_assessment_attempts::list_user_assessment_attempts;
use crate::application::learning::list_course_assessments::list_published_course_assessments;
use crate::application::learning::submit_assessment_attempt::{
    submit_assessment_attempt as submit_assessment_attempt_for_actor, AssessmentSubmissionError,
};
use crate::db;
use crate::db::schema::{course_join_requests, courses};
use crate::http::learning::dto::{
    AssessmentAttemptResponse, AssessmentResponse, SubmitAssessmentAttemptRequest,
    SubmitAssessmentAttemptResponse,
};
use crate::infra::postgres::learning::assessment_read_store::PostgresAssessmentReadStore;
use crate::infra::postgres::learning::assessment_submission_store::PostgresAssessmentSubmissionStore;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::course::{Course, UpdateCourse};
use crate::models::course_join_request::COURSE_JOIN_STATUS_APPROVED;
use crate::http::request_params::ParamType;
use crate::repositories::course_repository::assign_role_to_user_in_course;
use crate::services::course_enrollment_service::{
    decide_course_join_request as decide_course_join_request_for_actor,
    remove_course_enrollment as remove_course_enrollment_for_actor,
    request_course_join as request_course_join_for_actor, CourseEnrollmentError,
    CourseJoinDecisionRequest,
};
use crate::services::course_service::{
    create_course_with_invites_for_actor, discover_courses, discover_learner_course_catalog,
    discover_teacher_course_dashboard, get_learner_course_detail, get_learner_course_learning,
    get_learner_progress, get_teacher_course_enrollment_workspace, get_teacher_course_students,
    get_teacher_course_workspace, save_learner_progress, update_course_for_actor,
    CourseCreationError, CourseDiscoveryQuery, CourseLifecycleError, CourseLifecycleUpdateRequest,
    CourseUpdateError, LearnerCourseCatalogError, LearnerCourseCatalogQuery,
    TeacherCourseDashboardError, TeacherCourseDashboardQuery, TeacherCourseEnrollmentQuery,
};
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::{authenticated_user, authenticated_user_id};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

#[derive(Deserialize)]
pub struct CourseDiscoveryParams {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct LearnerCourseCatalogParams {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct TeacherCourseDashboardParams {
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct TeacherCourseEnrollmentParams {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn lifecycle_error_response(error: CourseLifecycleError) -> HttpResponse {
    match error {
        CourseLifecycleError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to update course status")
        }
        CourseLifecycleError::InvalidStatus(message) => HttpResponse::BadRequest().body(message),
        CourseLifecycleError::NotFound => HttpResponse::NotFound().body("Course not found"),
        CourseLifecycleError::Database(message) => {
            log::error!("event=course_lifecycle_update_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to update course status")
        }
    }
}

fn course_creation_error_response(error: CourseCreationError) -> HttpResponse {
    match error {
        CourseCreationError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to create course")
        }
        CourseCreationError::Database(message) => {
            log::error!("event=course_creation_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to create course")
        }
    }
}

fn course_update_error_response(error: CourseUpdateError) -> HttpResponse {
    match error {
        CourseUpdateError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to update course")
        }
        CourseUpdateError::NotFound => HttpResponse::NotFound().body("Course not found"),
        CourseUpdateError::Database(message) => {
            log::error!("event=course_update_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to update course")
        }
    }
}

fn course_enrollment_error_response(error: CourseEnrollmentError) -> HttpResponse {
    match error {
        CourseEnrollmentError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to manage enrollment")
        }
        CourseEnrollmentError::InvalidStatus(message) => HttpResponse::BadRequest().body(message),
        CourseEnrollmentError::NotFound => {
            HttpResponse::NotFound().body("Course enrollment not found")
        }
        CourseEnrollmentError::Database(message) => {
            log::error!("event=course_enrollment_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to manage course enrollment")
        }
    }
}

fn learner_course_catalog_error_response(error: LearnerCourseCatalogError) -> HttpResponse {
    match error {
        LearnerCourseCatalogError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to view course content")
        }
        LearnerCourseCatalogError::NotFound => HttpResponse::NotFound().body("Course not found"),
        LearnerCourseCatalogError::Database(message) => {
            log::error!("event=learner_course_catalog_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load course catalog")
        }
    }
}

fn teacher_course_dashboard_error_response(error: TeacherCourseDashboardError) -> HttpResponse {
    match error {
        TeacherCourseDashboardError::PermissionDenied(_) => HttpResponse::Forbidden()
            .body("User does not have permission to view this teaching course"),
        TeacherCourseDashboardError::NotFound => HttpResponse::NotFound().body("Course not found"),
        TeacherCourseDashboardError::Database(message) => {
            log::error!("event=teacher_course_dashboard_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load teaching courses")
        }
    }
}
