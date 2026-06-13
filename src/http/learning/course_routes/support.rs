use actix_web::HttpResponse;

use crate::application::learning::assessment::AssessmentReadError;
use crate::application::learning::create_course::CourseCreationError;
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionError;
use crate::application::learning::update_course::CourseUpdateError;
use crate::application::learning::update_course_lifecycle::CourseLifecycleError;
use crate::services::course_enrollment_service::CourseEnrollmentError;
use crate::services::course_service::{LearnerCourseCatalogError, TeacherCourseDashboardError};

pub(super) fn lifecycle_error_response(error: CourseLifecycleError) -> HttpResponse {
    match error {
        CourseLifecycleError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to update course status")
        }
        CourseLifecycleError::InvalidStatus(message) => HttpResponse::BadRequest().body(message),
        CourseLifecycleError::NotFound => HttpResponse::NotFound().body("Course not found"),
        CourseLifecycleError::Connection(message) => {
            log::error!("event=course_lifecycle_connection_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        CourseLifecycleError::Database(message) => {
            log::error!("event=course_lifecycle_update_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to update course status")
        }
    }
}

pub(super) fn course_creation_error_response(error: CourseCreationError) -> HttpResponse {
    match error {
        CourseCreationError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to create course")
        }
        CourseCreationError::Connection(message) => {
            log::error!("event=course_creation_connection_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        CourseCreationError::Database(message) => {
            log::error!("event=course_creation_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to create course")
        }
    }
}

pub(super) fn course_update_error_response(error: CourseUpdateError) -> HttpResponse {
    match error {
        CourseUpdateError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to update course")
        }
        CourseUpdateError::NotFound => HttpResponse::NotFound().body("Course not found"),
        CourseUpdateError::Connection(message) => {
            log::error!("event=course_update_connection_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        CourseUpdateError::Database(message) => {
            log::error!("event=course_update_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to update course")
        }
    }
}

pub(super) fn course_enrollment_error_response(error: CourseEnrollmentError) -> HttpResponse {
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

pub(super) fn learner_course_catalog_error_response(
    error: LearnerCourseCatalogError,
) -> HttpResponse {
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

pub(super) fn teacher_course_dashboard_error_response(
    error: TeacherCourseDashboardError,
) -> HttpResponse {
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

pub(super) fn assessment_read_error_log(error: &AssessmentReadError) -> String {
    match error {
        AssessmentReadError::Database(message) => message.clone(),
    }
}

pub(super) fn assessment_submission_error_log(error: &AssessmentSubmissionError) -> String {
    match error {
        AssessmentSubmissionError::NotFound => "not_found".to_string(),
        AssessmentSubmissionError::MaximumAttemptsReached => "maximum_attempts_reached".to_string(),
        AssessmentSubmissionError::LoadFailed(message)
        | AssessmentSubmissionError::SaveFailed(message) => message.clone(),
    }
}
