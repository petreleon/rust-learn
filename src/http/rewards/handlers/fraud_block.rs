use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::rewards::manage_fraud_block::{
    RewardFraudBlockAuditEventOutput, RewardFraudBlockUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{
    CreateRewardFraudBlockRequest, ListRewardFraudBlocksRequest, ListRewardFraudBlocksResponse,
    RewardFraudBlockAuditEventResponse, RewardFraudBlockResponse,
};
use crate::http::rewards::errors::reward_fraud_block_error;

pub async fn create_reward_fraud_block(
    requester: AuthUser,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
    body: web::Json<CreateRewardFraudBlockRequest>,
) -> Result<(web::Json<RewardFraudBlockResponse>, StatusCode), ApiError> {
    fraud_blocks
        .create_reward_fraud_block(requester.user_id(), body.into_inner().into())
        .await
        .map(RewardFraudBlockResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(reward_fraud_block_error)
}

pub async fn list_reward_fraud_blocks(
    requester: AuthUser,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
    query: web::Query<ListRewardFraudBlocksRequest>,
) -> Result<web::Json<ListRewardFraudBlocksResponse>, ApiError> {
    fraud_blocks
        .list_reward_fraud_blocks(requester.user_id(), query.into_inner().into())
        .await
        .map(ListRewardFraudBlocksResponse::from)
        .map(web::Json)
        .map_err(reward_fraud_block_error)
}

pub async fn revoke_reward_fraud_block(
    requester: AuthUser,
    path: web::Path<i64>,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
) -> Result<web::Json<RewardFraudBlockResponse>, ApiError> {
    fraud_blocks
        .revoke_reward_fraud_block(requester.user_id(), path.into_inner())
        .await
        .map(RewardFraudBlockResponse::from)
        .map(web::Json)
        .map_err(reward_fraud_block_error)
}

pub async fn reward_fraud_block_audit_history(
    requester: AuthUser,
    path: web::Path<i64>,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
) -> Result<web::Json<Vec<RewardFraudBlockAuditEventResponse>>, ApiError> {
    fraud_blocks
        .reward_fraud_block_audit_history(requester.user_id(), path.into_inner())
        .await
        .map(audit_event_responses)
        .map(web::Json)
        .map_err(reward_fraud_block_error)
}

fn audit_event_responses(
    events: Vec<RewardFraudBlockAuditEventOutput>,
) -> Vec<RewardFraudBlockAuditEventResponse> {
    events
        .into_iter()
        .map(RewardFraudBlockAuditEventResponse::from)
        .collect()
}
