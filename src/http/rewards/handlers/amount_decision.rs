use std::sync::Arc;

use actix_web::web;

use crate::application::rewards::decide_amount::RewardAmountDecisionUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{RewardAmountDecisionRequest, RewardAmountDecisionResponse};
use crate::http::rewards::errors::amount_decision_error;

pub async fn decide_reward_amount(
    requester: AuthUser,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn RewardAmountDecisionUseCase>>,
    body: web::Json<RewardAmountDecisionRequest>,
) -> Result<web::Json<RewardAmountDecisionResponse>, ApiError> {
    use_case
        .decide_reward_amount(
            requester.user_id(),
            path.into_inner(),
            body.into_inner().into(),
        )
        .await
        .map(RewardAmountDecisionResponse::from)
        .map(web::Json)
        .map_err(amount_decision_error)
}
