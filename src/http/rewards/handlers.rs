use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::manage_reward_policy::{
    RewardPolicyError, RewardPolicyOutput, RewardPolicyUseCase,
};
use crate::http::rewards::dto::{
    CreateRewardPolicyRequest, ListRewardPoliciesRequest, RewardPolicyResponse,
};
use crate::utils::request_auth::authenticated_user;

pub async fn create_reward_policy(
    req: HttpRequest,
    policies: web::Data<Arc<dyn RewardPolicyUseCase>>,
    body: web::Json<CreateRewardPolicyRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match policies
        .create_reward_policy(requester.user_id, body.into_inner().into())
        .await
    {
        Ok(policy) => HttpResponse::Created().json(RewardPolicyResponse::from(policy)),
        Err(error) => reward_policy_error_response(error),
    }
}

pub async fn list_reward_policies(
    req: HttpRequest,
    policies: web::Data<Arc<dyn RewardPolicyUseCase>>,
    query: web::Query<ListRewardPoliciesRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match policies
        .list_reward_policies(requester.user_id, query.into_inner().into())
        .await
    {
        Ok(policies) => HttpResponse::Ok().json(reward_policy_responses(policies)),
        Err(error) => reward_policy_error_response(error),
    }
}

fn reward_policy_error_response(error: RewardPolicyError) -> HttpResponse {
    match error {
        RewardPolicyError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward policy permission")
        }
        RewardPolicyError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        RewardPolicyError::NotFound => HttpResponse::NotFound().body("Reward policy not found"),
        RewardPolicyError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        RewardPolicyError::Database(message) => {
            log::error!("event=reward_policy_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward policy")
        }
    }
}

fn reward_policy_responses(policies: Vec<RewardPolicyOutput>) -> Vec<RewardPolicyResponse> {
    policies
        .into_iter()
        .map(RewardPolicyResponse::from)
        .collect()
}
