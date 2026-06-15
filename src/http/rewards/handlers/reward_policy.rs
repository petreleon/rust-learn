use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::rewards::manage_reward_policy::{RewardPolicyOutput, RewardPolicyUseCase};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{
    CreateRewardPolicyRequest, ListRewardPoliciesRequest, RewardPolicyResponse,
};
use crate::http::rewards::errors::reward_policy_error;

pub async fn create_reward_policy(
    requester: AuthUser,
    policies: web::Data<Arc<dyn RewardPolicyUseCase>>,
    body: web::Json<CreateRewardPolicyRequest>,
) -> Result<(web::Json<RewardPolicyResponse>, StatusCode), ApiError> {
    policies
        .create_reward_policy(requester.user_id(), body.into_inner().into())
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
    policies
        .list_reward_policies(requester.user_id(), query.into_inner().into())
        .await
        .map(reward_policy_responses)
        .map(web::Json)
        .map_err(reward_policy_error)
}

fn reward_policy_responses(policies: Vec<RewardPolicyOutput>) -> Vec<RewardPolicyResponse> {
    policies
        .into_iter()
        .map(RewardPolicyResponse::from)
        .collect()
}
