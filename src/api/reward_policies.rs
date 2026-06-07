use crate::db;
use crate::services::reward_policy_service::{
    self, CreateRewardPolicyRequest, ListRewardPoliciesRequest, RewardPolicyError,
};
use crate::utils::request_auth::authenticated_user;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

fn reward_policy_error_response(error: RewardPolicyError) -> HttpResponse {
    match error {
        RewardPolicyError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward policy permission")
        }
        RewardPolicyError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        RewardPolicyError::NotFound => HttpResponse::NotFound().body("Reward policy not found"),
        RewardPolicyError::Database(message) => {
            log::error!("event=reward_policy_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward policy")
        }
    }
}

async fn create_reward_policy(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<CreateRewardPolicyRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_policy_service::create_reward_policy(
        &mut conn,
        requester.user_id,
        body.into_inner(),
    )
    .await
    {
        Ok(policy) => HttpResponse::Created().json(policy),
        Err(error) => reward_policy_error_response(error),
    }
}

async fn list_reward_policies(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListRewardPoliciesRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_policy_service::list_reward_policies(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(policies) => HttpResponse::Ok().json(policies),
        Err(error) => reward_policy_error_response(error),
    }
}

pub fn reward_policy_scope() -> actix_web::Scope {
    web::scope("/reward-policies").service(
        web::resource("")
            .route(web::post().to(create_reward_policy))
            .route(web::get().to(list_reward_policies)),
    )
}
