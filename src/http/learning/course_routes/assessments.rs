use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::learning::assessment::AssessmentReadError;
use crate::application::learning::list_assessment_attempts::AssessmentAttemptsUseCase;
use crate::application::learning::list_course_assessments::CourseAssessmentsUseCase;
use crate::application::learning::submit_assessment_attempt::{
    AssessmentSubmissionError, AssessmentSubmissionUseCase,
};
use crate::http::learning::dto::{
    AssessmentAttemptResponse, AssessmentResponse, SubmitAssessmentAttemptRequest,
    SubmitAssessmentAttemptResponse,
};
use crate::utils::request_auth::authenticated_user_id;

use super::support::{assessment_read_error_log, assessment_submission_error_log};

pub(super) async fn list_course_assessments(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseAssessmentsUseCase>>,
) -> impl Responder {
    let course_id = path.into_inner();

    match use_case.list_published_course_assessments(course_id).await {
        Ok(list) => HttpResponse::Ok().json(
            list.into_iter()
                .map(AssessmentResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e @ AssessmentReadError::Connection(_)) => {
            log::error!(
                "event=assessments_list_connection_failed course_id={} error={}",
                course_id,
                assessment_read_error_log(&e)
            );
            HttpResponse::InternalServerError().body("DB unavailable")
        }
        Err(e) => {
            log::error!(
                "event=assessments_list_failed course_id={} error={}",
                course_id,
                assessment_read_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to list assessments")
        }
    }
}

pub(super) async fn submit_assessment_attempt(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<SubmitAssessmentAttemptRequest>,
    use_case: web::Data<Arc<dyn AssessmentSubmissionUseCase>>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let (course_id, assessment_id) = path.into_inner();
    let command = body
        .into_inner()
        .into_command(course_id, assessment_id, user_id);

    match use_case.submit_assessment_attempt(command).await {
        Ok(output) => HttpResponse::Ok().json(SubmitAssessmentAttemptResponse::from(output)),
        Err(AssessmentSubmissionError::NotFound) => {
            HttpResponse::NotFound().body("Assessment not found")
        }
        Err(AssessmentSubmissionError::MaximumAttemptsReached) => {
            HttpResponse::Forbidden().body("Maximum attempts reached")
        }
        Err(AssessmentSubmissionError::Connection(message)) => {
            log::error!(
                "event=assessment_submit_connection_failed assessment_id={} error={}",
                assessment_id,
                message
            );
            HttpResponse::InternalServerError().body("DB unavailable")
        }
        Err(AssessmentSubmissionError::LoadFailed(message)) => {
            log::error!(
                "event=assessment_submit_lookup_failed assessment_id={} error={}",
                assessment_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to load assessment")
        }
        Err(e) => {
            log::error!(
                "event=assessment_submit_failed assessment_id={} error={}",
                assessment_id,
                assessment_submission_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to save attempt")
        }
    }
}

pub(super) async fn list_assessment_attempts(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn AssessmentAttemptsUseCase>>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let (_course_id, assessment_id) = path.into_inner();

    match use_case
        .list_user_assessment_attempts(assessment_id, user_id)
        .await
    {
        Ok(attempts) => HttpResponse::Ok().json(
            attempts
                .into_iter()
                .map(AssessmentAttemptResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e @ AssessmentReadError::Connection(_)) => {
            log::error!(
                "event=assessment_attempts_connection_failed error={}",
                assessment_read_error_log(&e)
            );
            HttpResponse::InternalServerError().body("DB unavailable")
        }
        Err(e) => {
            log::error!(
                "event=assessment_attempts_failed error={}",
                assessment_read_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to load attempts")
        }
    }
}
