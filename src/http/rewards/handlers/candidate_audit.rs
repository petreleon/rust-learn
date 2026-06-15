use std::sync::Arc;

use actix_web::web;

use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditEvent, RewardCandidateAuditUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::RewardCandidateAuditEventResponse;
use crate::http::rewards::errors::reward_candidate_audit_error;

pub async fn list_reward_candidate_audit(
    requester: AuthUser,
    path: web::Path<i64>,
    audit: web::Data<Arc<dyn RewardCandidateAuditUseCase>>,
) -> Result<web::Json<Vec<RewardCandidateAuditEventResponse>>, ApiError> {
    audit
        .list_reward_candidate_audit(requester.user_id(), path.into_inner())
        .await
        .map(audit_event_responses)
        .map(web::Json)
        .map_err(reward_candidate_audit_error)
}

fn audit_event_responses(
    events: Vec<RewardCandidateAuditEvent>,
) -> Vec<RewardCandidateAuditEventResponse> {
    events
        .into_iter()
        .map(RewardCandidateAuditEventResponse::from)
        .collect()
}
