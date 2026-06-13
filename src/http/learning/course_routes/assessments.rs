use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::learning::list_assessment_attempts::list_user_assessment_attempts;
use crate::application::learning::list_course_assessments::list_published_course_assessments;
use crate::application::learning::submit_assessment_attempt::{
    submit_assessment_attempt as submit_assessment_attempt_for_actor, AssessmentSubmissionError,
};
use crate::db;
use crate::http::learning::dto::{
    AssessmentAttemptResponse, AssessmentResponse, SubmitAssessmentAttemptRequest,
    SubmitAssessmentAttemptResponse,
};
use crate::infra::postgres::learning::assessment_read_store::PostgresAssessmentReadStore;
use crate::infra::postgres::learning::assessment_submission_store::PostgresAssessmentSubmissionStore;
use crate::utils::request_auth::authenticated_user_id;

use super::support::{assessment_read_error_log, assessment_submission_error_log};

pub(super) async fn list_course_assessments(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };
    let mut store = PostgresAssessmentReadStore::new(&mut conn);

    match list_published_course_assessments(&mut store, course_id).await {
        Ok(list) => HttpResponse::Ok().json(
            list.into_iter()
                .map(AssessmentResponse::from)
                .collect::<Vec<_>>(),
        ),
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
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let (course_id, assessment_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };
    let mut store = PostgresAssessmentSubmissionStore::new(&mut conn);
    let command = body
        .into_inner()
        .into_command(course_id, assessment_id, user_id);

    match submit_assessment_attempt_for_actor(&mut store, command).await {
        Ok(output) => HttpResponse::Ok().json(SubmitAssessmentAttemptResponse::from(output)),
        Err(AssessmentSubmissionError::NotFound) => {
            HttpResponse::NotFound().body("Assessment not found")
        }
        Err(AssessmentSubmissionError::MaximumAttemptsReached) => {
            HttpResponse::Forbidden().body("Maximum attempts reached")
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
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let (_course_id, assessment_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };
    let mut store = PostgresAssessmentReadStore::new(&mut conn);

    match list_user_assessment_attempts(&mut store, assessment_id, user_id).await {
        Ok(attempts) => HttpResponse::Ok().json(
            attempts
                .into_iter()
                .map(AssessmentAttemptResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => {
            log::error!(
                "event=assessment_attempts_failed error={}",
                assessment_read_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to load attempts")
        }
    }
}
