use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsListQuery, CourseCompletionTermsUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::learning::dto::{
    CourseCompletionTermsDecisionRequest, CourseCompletionTermsHistoryResponse,
    CourseCompletionTermsProposalRequest, CourseCompletionTermsResponse,
};

use super::errors::course_completion_terms_error;

pub(super) async fn list_course_completion_terms(
    actor: AuthUserId,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseCompletionTermsUseCase>>,
) -> Result<web::Json<CourseCompletionTermsHistoryResponse>, ApiError> {
    let course_id = path.into_inner();
    use_case
        .list_course_completion_terms(CourseCompletionTermsListQuery {
            actor_user_id: actor.into_inner(),
            course_id,
        })
        .await
        .map(CourseCompletionTermsHistoryResponse::from)
        .map(web::Json)
        .map_err(course_completion_terms_error)
}

pub(super) async fn submit_course_completion_terms(
    actor: AuthUserId,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseCompletionTermsUseCase>>,
    body: web::Json<CourseCompletionTermsProposalRequest>,
) -> Result<(web::Json<CourseCompletionTermsResponse>, StatusCode), ApiError> {
    let actor_user_id = actor.into_inner();
    let course_id = path.into_inner();
    use_case
        .submit_course_completion_terms(
            body.into_inner()
                .into_proposal_command(actor_user_id, course_id),
        )
        .await
        .map(CourseCompletionTermsResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(course_completion_terms_error)
}

pub(super) async fn counter_course_completion_terms(
    actor: AuthUserId,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseCompletionTermsUseCase>>,
    body: web::Json<CourseCompletionTermsProposalRequest>,
) -> Result<web::Json<CourseCompletionTermsResponse>, ApiError> {
    let actor_user_id = actor.into_inner();
    let (course_id, terms_id) = path.into_inner();
    use_case
        .counter_course_completion_terms(body.into_inner().into_counter_command(
            actor_user_id,
            course_id,
            terms_id,
        ))
        .await
        .map(CourseCompletionTermsResponse::from)
        .map(web::Json)
        .map_err(course_completion_terms_error)
}

pub(super) async fn accept_course_completion_terms(
    actor: AuthUserId,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseCompletionTermsUseCase>>,
    body: web::Json<CourseCompletionTermsDecisionRequest>,
) -> Result<web::Json<CourseCompletionTermsResponse>, ApiError> {
    decide_terms(actor, path, use_case, body, TermsDecision::Accept).await
}

pub(super) async fn reject_course_completion_terms(
    actor: AuthUserId,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseCompletionTermsUseCase>>,
    body: web::Json<CourseCompletionTermsDecisionRequest>,
) -> Result<web::Json<CourseCompletionTermsResponse>, ApiError> {
    decide_terms(actor, path, use_case, body, TermsDecision::Reject).await
}

pub(super) async fn withdraw_course_completion_terms(
    actor: AuthUserId,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseCompletionTermsUseCase>>,
    body: web::Json<CourseCompletionTermsDecisionRequest>,
) -> Result<web::Json<CourseCompletionTermsResponse>, ApiError> {
    decide_terms(actor, path, use_case, body, TermsDecision::Withdraw).await
}

enum TermsDecision {
    Accept,
    Reject,
    Withdraw,
}

async fn decide_terms(
    actor: AuthUserId,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseCompletionTermsUseCase>>,
    body: web::Json<CourseCompletionTermsDecisionRequest>,
    decision: TermsDecision,
) -> Result<web::Json<CourseCompletionTermsResponse>, ApiError> {
    let actor_user_id = actor.into_inner();
    let (course_id, terms_id) = path.into_inner();
    let command = body
        .into_inner()
        .into_command(actor_user_id, course_id, terms_id);
    let result = match decision {
        TermsDecision::Accept => use_case.accept_course_completion_terms(command).await,
        TermsDecision::Reject => use_case.reject_course_completion_terms(command).await,
        TermsDecision::Withdraw => use_case.withdraw_course_completion_terms(command).await,
    };
    result
        .map(CourseCompletionTermsResponse::from)
        .map(web::Json)
        .map_err(course_completion_terms_error)
}
