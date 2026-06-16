use std::sync::Arc;

use actix_web::web;

use crate::application::learning::list_assessment_attempts::AssessmentAttemptsUseCase;
use crate::application::learning::list_course_assessments::CourseAssessmentsUseCase;
use crate::application::learning::manage_assessments::AssessmentAuthoringUseCase;
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::learning::dto::{
    AssessmentAttemptResponse, AssessmentAuthoringRequest, AssessmentAuthoringResponse,
    AssessmentResponse, SubmitAssessmentAttemptRequest, SubmitAssessmentAttemptResponse,
};

use super::errors::{
    assessment_attempts_error, assessment_authoring_error, assessment_submission_error,
    course_assessments_error,
};

pub(super) async fn list_course_assessments(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseAssessmentsUseCase>>,
) -> Result<web::Json<Vec<AssessmentResponse>>, ApiError> {
    let course_id = path.into_inner();

    use_case
        .list_published_course_assessments(course_id)
        .await
        .map(assessment_responses)
        .map(web::Json)
        .map_err(|error| course_assessments_error(course_id, error))
}

pub(super) async fn list_authoring_assessments(
    user: AuthUserId,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn AssessmentAuthoringUseCase>>,
) -> Result<web::Json<Vec<AssessmentAuthoringResponse>>, ApiError> {
    let actor_user_id = user.into_inner();
    let course_id = path.into_inner();

    use_case
        .list_course_assessments_for_authoring(actor_user_id, course_id)
        .await
        .map(authoring_responses)
        .map(web::Json)
        .map_err(|error| assessment_authoring_error(course_id, error))
}

pub(super) async fn create_authoring_assessment(
    user: AuthUserId,
    path: web::Path<i32>,
    body: web::Json<AssessmentAuthoringRequest>,
    use_case: web::Data<Arc<dyn AssessmentAuthoringUseCase>>,
) -> Result<web::Json<AssessmentAuthoringResponse>, ApiError> {
    let actor_user_id = user.into_inner();
    let course_id = path.into_inner();
    let command = body
        .into_inner()
        .into_create_command(actor_user_id, course_id);

    use_case
        .create_assessment(command)
        .await
        .map(AssessmentAuthoringResponse::from)
        .map(web::Json)
        .map_err(|error| assessment_authoring_error(course_id, error))
}

pub(super) async fn update_authoring_assessment(
    user: AuthUserId,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssessmentAuthoringRequest>,
    use_case: web::Data<Arc<dyn AssessmentAuthoringUseCase>>,
) -> Result<web::Json<AssessmentAuthoringResponse>, ApiError> {
    let actor_user_id = user.into_inner();
    let (course_id, assessment_id) = path.into_inner();
    let command = body
        .into_inner()
        .into_update_command(actor_user_id, course_id, assessment_id);

    use_case
        .update_assessment(command)
        .await
        .map(AssessmentAuthoringResponse::from)
        .map(web::Json)
        .map_err(|error| assessment_authoring_error(course_id, error))
}

pub(super) async fn submit_assessment_attempt(
    user: AuthUserId,
    path: web::Path<(i32, i32)>,
    body: web::Json<SubmitAssessmentAttemptRequest>,
    use_case: web::Data<Arc<dyn AssessmentSubmissionUseCase>>,
) -> Result<web::Json<SubmitAssessmentAttemptResponse>, ApiError> {
    let user_id = user.into_inner();
    let (course_id, assessment_id) = path.into_inner();
    let command = body
        .into_inner()
        .into_command(course_id, assessment_id, user_id);

    use_case
        .submit_assessment_attempt(command)
        .await
        .map(SubmitAssessmentAttemptResponse::from)
        .map(web::Json)
        .map_err(|error| assessment_submission_error(assessment_id, error))
}

pub(super) async fn list_assessment_attempts(
    user: AuthUserId,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn AssessmentAttemptsUseCase>>,
) -> Result<web::Json<Vec<AssessmentAttemptResponse>>, ApiError> {
    let user_id = user.into_inner();
    let (_course_id, assessment_id) = path.into_inner();

    use_case
        .list_user_assessment_attempts(assessment_id, user_id)
        .await
        .map(attempt_responses)
        .map(web::Json)
        .map_err(assessment_attempts_error)
}

fn assessment_responses(
    list: Vec<crate::application::learning::assessment::AssessmentOutput>,
) -> Vec<AssessmentResponse> {
    list.into_iter().map(AssessmentResponse::from).collect()
}

fn authoring_responses(
    list: Vec<crate::application::learning::manage_assessments::AuthoredAssessmentOutput>,
) -> Vec<AssessmentAuthoringResponse> {
    list.into_iter()
        .map(AssessmentAuthoringResponse::from)
        .collect()
}

fn attempt_responses(
    list: Vec<crate::application::learning::assessment::AssessmentAttemptOutput>,
) -> Vec<AssessmentAttemptResponse> {
    list.into_iter()
        .map(AssessmentAttemptResponse::from)
        .collect()
}
