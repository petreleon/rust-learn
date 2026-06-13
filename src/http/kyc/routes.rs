use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::kyc::{
    KycAuditQuery, KycAuditUseCase, KycDecisionCommand, KycError, KycReviewQueueOutput,
    KycReviewUseCase, KycStatusUseCase, KycSubmissionUseCase, SubmitKycCommand,
};
use crate::http::extractors::auth_user::AuthUser;

use super::dto::{
    KycAuditEventResponse, KycDecisionRequest, KycReviewQueueResponse, KycStatusResponse,
    KycSubmissionResponse, SubmitKycRequest,
};

fn service_error_response(error: KycError) -> HttpResponse {
    match error {
        KycError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        KycError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        KycError::InvalidTransition(message) => HttpResponse::Conflict().body(message),
        KycError::NotFound => HttpResponse::NotFound().body("KYC submission not found"),
        KycError::Connection(message) | KycError::Database(message) => {
            log::error!("event=kyc_api_failed reason=database error={}", message);
            HttpResponse::InternalServerError().body("Failed to process KYC request")
        }
    }
}

async fn get_my_kyc(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycStatusUseCase>>,
) -> impl Responder {
    match use_case.get_my_status(requester.user_id()).await {
        Ok(status) => HttpResponse::Ok().json(KycStatusResponse::from(status)),
        Err(error) => service_error_response(error),
    }
}

async fn submit_my_kyc(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycSubmissionUseCase>>,
    body: web::Json<SubmitKycRequest>,
) -> impl Responder {
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
    match use_case.submit_my_kyc(command).await {
        Ok(status) => HttpResponse::Created().json(KycStatusResponse::from(status)),
        Err(error) => service_error_response(error),
    }
}

async fn list_review_queue(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycReviewUseCase>>,
) -> impl Responder {
    match use_case.list_review_queue(requester.user_id()).await {
        Ok(queue) => HttpResponse::Ok().json(review_queue_response(queue)),
        Err(error) => service_error_response(error),
    }
}

async fn decide_kyc(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycReviewUseCase>>,
    path: web::Path<i64>,
    body: web::Json<KycDecisionRequest>,
) -> impl Responder {
    let request = body.into_inner();
    let command = KycDecisionCommand {
        rejection_reason: request.rejection_reason,
        reviewer_user_id: requester.user_id(),
        status: request.status,
        submission_id: path.into_inner(),
    };
    match use_case.decide_submission(command).await {
        Ok(submission) => HttpResponse::Ok().json(KycSubmissionResponse::from(submission)),
        Err(error) => service_error_response(error),
    }
}

async fn list_kyc_audit(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn KycAuditUseCase>>,
    path: web::Path<i64>,
) -> impl Responder {
    let query = KycAuditQuery {
        reviewer_user_id: requester.user_id(),
        submission_id: path.into_inner(),
    };
    match use_case.list_submission_audit(query).await {
        Ok(events) => HttpResponse::Ok().json(audit_event_responses(events)),
        Err(error) => service_error_response(error),
    }
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
