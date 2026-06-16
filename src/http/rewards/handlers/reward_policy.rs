use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::rewards::manage_reward_policy::{
    RewardPolicyAuditEventOutput, RewardPolicyOutput, RewardPolicyUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{
    CreateRewardPolicyRequest, ListRewardPoliciesRequest, RewardPolicyAuditEventResponse,
    RewardPolicyResponse, UpdateRewardPolicyActivationRequest,
};
use crate::http::rewards::errors::reward_policy_error;

pub async fn create_reward_policy(
    requester: AuthUser,
    policies: web::Data<Arc<dyn RewardPolicyUseCase>>,
    body: web::Json<CreateRewardPolicyRequest>,
) -> Result<(web::Json<RewardPolicyResponse>, StatusCode), ApiError> {
    let command = body
        .into_inner()
        .into_command()
        .map_err(reward_policy_error)?;

    policies
        .create_reward_policy(requester.user_id(), command)
        .await
        .map(RewardPolicyResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(reward_policy_error)
}

pub async fn list_reward_policies(
    requester: AuthUser,
    policies: web::Data<Arc<dyn RewardPolicyUseCase>>,
    query: web::Query<ListRewardPoliciesRequest>,
) -> Result<web::Json<Vec<RewardPolicyResponse>>, ApiError> {
    let query = query
        .into_inner()
        .into_query()
        .map_err(reward_policy_error)?;

    policies
        .list_reward_policies(requester.user_id(), query)
        .await
        .map(reward_policy_responses)
        .map(web::Json)
        .map_err(reward_policy_error)
}

pub async fn update_reward_policy_activation(
    requester: AuthUser,
    policies: web::Data<Arc<dyn RewardPolicyUseCase>>,
    path: web::Path<i64>,
    body: web::Json<UpdateRewardPolicyActivationRequest>,
) -> Result<web::Json<RewardPolicyResponse>, ApiError> {
    let command = body.into_inner().into_command(path.into_inner());

    policies
        .update_reward_policy_activation(requester.user_id(), command)
        .await
        .map(RewardPolicyResponse::from)
        .map(web::Json)
        .map_err(reward_policy_error)
}

pub async fn list_reward_policy_audit(
    requester: AuthUser,
    policies: web::Data<Arc<dyn RewardPolicyUseCase>>,
    path: web::Path<i64>,
) -> Result<web::Json<Vec<RewardPolicyAuditEventResponse>>, ApiError> {
    policies
        .list_reward_policy_audit(requester.user_id(), path.into_inner())
        .await
        .map(reward_policy_audit_responses)
        .map(web::Json)
        .map_err(reward_policy_error)
}

fn reward_policy_responses(policies: Vec<RewardPolicyOutput>) -> Vec<RewardPolicyResponse> {
    policies
        .into_iter()
        .map(RewardPolicyResponse::from)
        .collect()
}

fn reward_policy_audit_responses(
    events: Vec<RewardPolicyAuditEventOutput>,
) -> Vec<RewardPolicyAuditEventResponse> {
    events
        .into_iter()
        .map(RewardPolicyAuditEventResponse::from)
        .collect()
}
