use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::manage_fraud_block::{
    RewardFraudBlockAuditEventOutput, RewardFraudBlockError, RewardFraudBlockUseCase,
};
use crate::http::extractors::request_auth::authenticated_user;
use crate::http::rewards::dto::{
    CreateRewardFraudBlockRequest, ListRewardFraudBlocksRequest, ListRewardFraudBlocksResponse,
    RewardFraudBlockAuditEventResponse, RewardFraudBlockResponse,
};

pub async fn create_reward_fraud_block(
    req: HttpRequest,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
    body: web::Json<CreateRewardFraudBlockRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match fraud_blocks
        .create_reward_fraud_block(requester.user_id, body.into_inner().into())
        .await
    {
        Ok(block) => HttpResponse::Created().json(RewardFraudBlockResponse::from(block)),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

pub async fn list_reward_fraud_blocks(
    req: HttpRequest,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
    query: web::Query<ListRewardFraudBlocksRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match fraud_blocks
        .list_reward_fraud_blocks(requester.user_id, query.into_inner().into())
        .await
    {
        Ok(output) => HttpResponse::Ok().json(ListRewardFraudBlocksResponse::from(output)),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

pub async fn revoke_reward_fraud_block(
    req: HttpRequest,
    path: web::Path<i64>,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match fraud_blocks
        .revoke_reward_fraud_block(requester.user_id, path.into_inner())
        .await
    {
        Ok(block) => HttpResponse::Ok().json(RewardFraudBlockResponse::from(block)),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

pub async fn reward_fraud_block_audit_history(
    req: HttpRequest,
    path: web::Path<i64>,
    fraud_blocks: web::Data<Arc<dyn RewardFraudBlockUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match fraud_blocks
        .reward_fraud_block_audit_history(requester.user_id, path.into_inner())
        .await
    {
        Ok(events) => HttpResponse::Ok().json(audit_event_responses(events)),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

fn reward_fraud_block_error_response(error: RewardFraudBlockError) -> HttpResponse {
    match error {
        RewardFraudBlockError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward fraud-block permission")
        }
        RewardFraudBlockError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        RewardFraudBlockError::NotFound => {
            HttpResponse::NotFound().body("Reward fraud block not found")
        }
        RewardFraudBlockError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        RewardFraudBlockError::Database(message) => {
            log::error!("event=reward_fraud_block_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward fraud block")
        }
    }
}

fn audit_event_responses(
    events: Vec<RewardFraudBlockAuditEventOutput>,
) -> Vec<RewardFraudBlockAuditEventResponse> {
    events
        .into_iter()
        .map(RewardFraudBlockAuditEventResponse::from)
        .collect()
}
