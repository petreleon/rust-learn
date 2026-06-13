use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidatesError, PlatformRewardCandidatesUseCase,
};
use crate::http::rewards::dto::{
    PlatformRewardCandidatesRequest, PlatformRewardCandidatesResponseBody,
};
use crate::utils::request_auth::authenticated_user;

pub async fn list_platform_reward_candidates(
    req: HttpRequest,
    candidates: web::Data<Arc<dyn PlatformRewardCandidatesUseCase>>,
    query: web::Query<PlatformRewardCandidatesRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    match candidates
        .list_platform_reward_candidates(requester.user_id, query.into_inner().into())
        .await
    {
        Ok(output) => HttpResponse::Ok().json(PlatformRewardCandidatesResponseBody::from(output)),
        Err(error) => platform_reward_candidates_error_response(error),
    }
}

fn platform_reward_candidates_error_response(error: PlatformRewardCandidatesError) -> HttpResponse {
    match error {
        PlatformRewardCandidatesError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have reward candidate permission")
        }
        PlatformRewardCandidatesError::InvalidStatus(message) => {
            HttpResponse::Conflict().body(message)
        }
        PlatformRewardCandidatesError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        PlatformRewardCandidatesError::Database(message) => {
            log::error!(
                "event=platform_reward_candidates_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process reward candidate")
        }
    }
}
