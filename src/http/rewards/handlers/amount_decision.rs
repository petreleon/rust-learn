use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::decide_amount::{
    RewardAmountDecisionError, RewardAmountDecisionUseCase,
};
use crate::http::extractors::request_auth::authenticated_user;
use crate::http::rewards::dto::{RewardAmountDecisionRequest, RewardAmountDecisionResponse};

pub async fn decide_reward_amount(
    req: HttpRequest,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn RewardAmountDecisionUseCase>>,
    body: web::Json<RewardAmountDecisionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match use_case
        .decide_reward_amount(
            requester.user_id,
            path.into_inner(),
            body.into_inner().into(),
        )
        .await
    {
        Ok(candidate) => HttpResponse::Ok().json(RewardAmountDecisionResponse::from(candidate)),
        Err(error) => amount_decision_error_response(error),
    }
}

fn amount_decision_error_response(error: RewardAmountDecisionError) -> HttpResponse {
    match error {
        RewardAmountDecisionError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward candidate permission")
        }
        RewardAmountDecisionError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        RewardAmountDecisionError::InvalidStatus(message) => HttpResponse::Conflict().body(message),
        RewardAmountDecisionError::NotFound => {
            HttpResponse::NotFound().body("Reward candidate not found")
        }
        RewardAmountDecisionError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        RewardAmountDecisionError::Database(message) => {
            log::error!("event=reward_candidate_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process reward candidate")
        }
    }
}
