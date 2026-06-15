use super::{
    assessment_submission_error, course_creation_error, course_enrollment_error,
    course_role_assignment_error, learner_course_read_error,
};
use crate::application::learning::assign_course_role::CourseRoleAssignmentError;
use crate::application::learning::course_enrollment::CourseEnrollmentError;
use crate::application::learning::create_course::CourseCreationError;
use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionError;
use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
use serde_json::Value;

#[actix_web::test]
async fn course_creation_permission_denied_uses_api_error_envelope() {
    let body = parse_body(
        course_creation_error(CourseCreationError::PermissionDenied(
            "create_course".to_string(),
        ))
        .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::FORBIDDEN);
    assert_eq!(body.value["error"]["code"], "permission_denied");
    assert_eq!(
        body.value["error"]["message"],
        "User does not have permission to create course"
    );
}

#[actix_web::test]
async fn learner_course_not_found_uses_course_code() {
    let body =
        parse_body(learner_course_read_error(LearnerCourseCatalogError::NotFound).error_response())
            .await;

    assert_eq!(body.status, StatusCode::NOT_FOUND);
    assert_eq!(body.value["error"]["code"], "course_not_found");
    assert_eq!(body.value["error"]["message"], "Course not found");
}

#[actix_web::test]
async fn enrollment_invalid_status_is_bad_request() {
    let body = parse_body(
        course_enrollment_error(CourseEnrollmentError::InvalidStatus("closed".to_string()))
            .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::BAD_REQUEST);
    assert_eq!(body.value["error"]["code"], "invalid_input");
    assert_eq!(body.value["error"]["message"], "closed");
}

#[actix_web::test]
async fn role_hierarchy_violation_is_forbidden() {
    let body = parse_body(
        course_role_assignment_error(CourseRoleAssignmentError::HierarchyViolation)
            .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::FORBIDDEN);
    assert_eq!(body.value["error"]["code"], "permission_denied");
}

#[actix_web::test]
async fn assessment_not_found_uses_specific_code() {
    let body = parse_body(
        assessment_submission_error(9, AssessmentSubmissionError::NotFound).error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::NOT_FOUND);
    assert_eq!(body.value["error"]["code"], "assessment_not_found");
    assert_eq!(body.value["error"]["message"], "Assessment not found");
}

struct ParsedErrorBody {
    status: StatusCode,
    value: Value,
}

async fn parse_body(response: actix_web::HttpResponse) -> ParsedErrorBody {
    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let value = serde_json::from_slice(&body).unwrap();
    ParsedErrorBody { status, value }
}
