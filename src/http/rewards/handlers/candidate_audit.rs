use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent, RewardCandidateAuditUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::RewardCandidateAuditEventResponse;

pub async fn list_reward_candidate_audit(
    requester: AuthUser,
    path: web::Path<i64>,
    audit: web::Data<Arc<dyn RewardCandidateAuditUseCase>>,
) -> impl Responder {
    match audit
        .list_reward_candidate_audit(requester.user_id(), path.into_inner())
        .await
    {
        Ok(events) => HttpResponse::Ok().json(audit_event_responses(events)),
        Err(error) => reward_candidate_audit_error_response(error),
    }
}

fn reward_candidate_audit_error_response(error: RewardCandidateAuditError) -> HttpResponse {
    match error {
        RewardCandidateAuditError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward candidate permission")
        }
        RewardCandidateAuditError::NotFound => {
            HttpResponse::NotFound().body("Reward candidate not found")
        }
        RewardCandidateAuditError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        RewardCandidateAuditError::Database(message) => {
            log::error!("event=reward_candidate_audit_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward candidate")
        }
    }
}

fn audit_event_responses(
    events: Vec<RewardCandidateAuditEvent>,
) -> Vec<RewardCandidateAuditEventResponse> {
    events
        .into_iter()
        .map(RewardCandidateAuditEventResponse::from)
        .collect()
}
