use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::kyc::{
    KycAuditQuery, KycAuditUseCase, KycDecisionCommand, KycReviewQueueOutput, KycReviewUseCase,
    KycStatusUseCase, KycSubmissionUseCase, SubmitKycCommand,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;

use super::dto::{
    KycAuditEventResponse, KycDecisionRequest, KycReviewQueueResponse, KycStatusResponse,
    KycSubmissionResponse, SubmitKycRequest,
};
use super::errors::kyc_error;

async fn get_my_kyc(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycStatusUseCase>>,
) -> Result<web::Json<KycStatusResponse>, ApiError> {
    use_case
        .get_my_status(requester.user_id())
        .await
        .map(KycStatusResponse::from)
        .map(web::Json)
        .map_err(kyc_error)
}

async fn submit_my_kyc(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycSubmissionUseCase>>,
    body: web::Json<SubmitKycRequest>,
) -> Result<(web::Json<KycStatusResponse>, StatusCode), ApiError> {
    let request = body.into_inner();
    let command = SubmitKycCommand {
        country_code: request.country_code,
        document_last4: request.document_last4,
        document_type: request.document_type,
        evidence_reference: request.evidence_reference,
        legal_name: request.legal_name,
        provider_reference: request.provider_reference,
        user_id: requester.user_id(),
    };
    use_case
        .submit_my_kyc(command)
        .await
        .map(KycStatusResponse::from)
        .map(web::Json)
        .map(|body| (body, StatusCode::CREATED))
        .map_err(kyc_error)
}

async fn list_review_queue(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycReviewUseCase>>,
) -> Result<web::Json<KycReviewQueueResponse>, ApiError> {
    use_case
        .list_review_queue(requester.user_id())
        .await
        .map(review_queue_response)
        .map(web::Json)
        .map_err(kyc_error)
}

async fn decide_kyc(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycReviewUseCase>>,
    path: web::Path<i64>,
    body: web::Json<KycDecisionRequest>,
) -> Result<web::Json<KycSubmissionResponse>, ApiError> {
    let request = body.into_inner();
    let command = KycDecisionCommand {
        rejection_reason: request.rejection_reason,
        reviewer_user_id: requester.user_id(),
        status: request.status,
        submission_id: path.into_inner(),
    };
    use_case
        .decide_submission(command)
        .await
        .map(KycSubmissionResponse::from)
        .map(web::Json)
        .map_err(kyc_error)
}

async fn list_kyc_audit(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycAuditUseCase>>,
    path: web::Path<i64>,
) -> Result<web::Json<Vec<KycAuditEventResponse>>, ApiError> {
    let query = KycAuditQuery {
        reviewer_user_id: requester.user_id(),
        submission_id: path.into_inner(),
    };
    use_case
        .list_submission_audit(query)
        .await
        .map(audit_event_responses)
        .map(web::Json)
        .map_err(kyc_error)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(kyc_scope());
}

fn kyc_scope() -> actix_web::Scope {
    web::scope("/kyc")
        .service(
            web::resource("/me")
                .route(web::get().to(get_my_kyc))
                .route(web::post().to(submit_my_kyc)),
        )
        .service(web::resource("/review").route(web::get().to(list_review_queue)))
        .service(web::resource("/review/{id}").route(web::put().to(decide_kyc)))
        .service(web::resource("/review/{id}/audit").route(web::get().to(list_kyc_audit)))
}

fn review_queue_response(queue: KycReviewQueueOutput) -> KycReviewQueueResponse {
    KycReviewQueueResponse {
        submissions: queue.submissions.into_iter().map(Into::into).collect(),
    }
}

fn audit_event_responses(
    events: Vec<crate::application::kyc::KycAuditEventOutput>,
) -> Vec<KycAuditEventResponse> {
    events.into_iter().map(Into::into).collect()
}
