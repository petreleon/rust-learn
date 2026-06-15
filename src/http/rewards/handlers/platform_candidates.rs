use std::sync::Arc;

use actix_web::web;

use crate::application::rewards::list_platform_candidates::PlatformRewardCandidatesUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{
    PlatformRewardCandidatesRequest, PlatformRewardCandidatesResponseBody,
};
use crate::http::rewards::errors::platform_reward_candidates_error;

pub async fn list_platform_reward_candidates(
    requester: AuthUser,
    candidates: web::Data<Arc<dyn PlatformRewardCandidatesUseCase>>,
    query: web::Query<PlatformRewardCandidatesRequest>,
) -> Result<web::Json<PlatformRewardCandidatesResponseBody>, ApiError> {
    candidates
        .list_platform_reward_candidates(requester.user_id(), query.into_inner().into())
        .await
        .map(PlatformRewardCandidatesResponseBody::from)
        .map(web::Json)
        .map_err(platform_reward_candidates_error)
}
