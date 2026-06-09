use crate::db;
use crate::services::reward_fraud_block_service::{
    self, ListRewardFraudBlocksRequest, RewardFraudBlockError, RewardFraudBlockRequest,
};
use crate::utils::request_auth::authenticated_user;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

fn reward_fraud_block_error_response(error: RewardFraudBlockError) -> HttpResponse {
    match error {
        RewardFraudBlockError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward fraud-block permission")
        }
        RewardFraudBlockError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        RewardFraudBlockError::NotFound => {
            HttpResponse::NotFound().body("Reward fraud block not found")
        }
        RewardFraudBlockError::Database(message) => {
            log::error!("event=reward_fraud_block_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward fraud block")
        }
    }
}

async fn create_reward_fraud_block(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<RewardFraudBlockRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_fraud_block_service::create_reward_fraud_block(
        &mut conn,
        requester.user_id,
        body.into_inner(),
    )
    .await
    {
        Ok(block) => HttpResponse::Created().json(block),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

async fn list_reward_fraud_blocks(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListRewardFraudBlocksRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_fraud_block_service::list_reward_fraud_blocks(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

async fn revoke_reward_fraud_block(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_fraud_block_service::revoke_reward_fraud_block(
        &mut conn,
        requester.user_id,
        path.into_inner(),
    )
    .await
    {
        Ok(block) => HttpResponse::Ok().json(block),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

async fn reward_fraud_block_audit_history(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_fraud_block_service::reward_fraud_block_audit_history(
        &mut conn,
        requester.user_id,
        path.into_inner(),
    )
    .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(error) => reward_fraud_block_error_response(error),
    }
}

pub fn reward_fraud_block_scope() -> actix_web::Scope {
    web::scope("/reward-fraud-blocks")
        .service(
            web::resource("")
                .route(web::post().to(create_reward_fraud_block))
                .route(web::get().to(list_reward_fraud_blocks)),
        )
        .service(web::resource("/{id}/revoke").route(web::put().to(revoke_reward_fraud_block)))
        .service(
            web::resource("/{id}/audit").route(web::get().to(reward_fraud_block_audit_history)),
        )
}
