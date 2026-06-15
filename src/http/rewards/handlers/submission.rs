use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::rewards::submit_candidate::RewardCandidateSubmissionUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::rewards::dto::{RewardCandidateSubmissionResponse, SubmitRewardCandidateRequest};
use crate::http::rewards::errors::reward_candidate_submission_error;

pub async fn submit_course_reward_candidate(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn RewardCandidateSubmissionUseCase>>,
    body: web::Json<SubmitRewardCandidateRequest>,
) -> Result<(web::Json<RewardCandidateSubmissionResponse>, StatusCode), ApiError> {
    let command = body
        .into_inner()
        .into_command()
        .map_err(reward_candidate_submission_error)?;

    use_case
        .submit_course_reward_candidate(requester.user_id(), path.into_inner(), command)
        .await
        .map(RewardCandidateSubmissionResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(reward_candidate_submission_error)
}

pub async fn submit_organization_reward_candidate(
    requester: AuthUser,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn RewardCandidateSubmissionUseCase>>,
    body: web::Json<SubmitRewardCandidateRequest>,
) -> Result<(web::Json<RewardCandidateSubmissionResponse>, StatusCode), ApiError> {
    let (organization_id, course_id) = path.into_inner();
    let command = body
        .into_inner()
        .into_command()
        .map_err(reward_candidate_submission_error)?;

    use_case
        .submit_organization_reward_candidate(
            requester.user_id(),
            organization_id,
            course_id,
            command,
        )
        .await
        .map(RewardCandidateSubmissionResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(reward_candidate_submission_error)
}
