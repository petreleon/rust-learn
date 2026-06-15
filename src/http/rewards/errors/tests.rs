use super::{reward_candidate_submission_error, reward_fraud_block_error, reward_policy_error};
use crate::application::rewards::manage_fraud_block::RewardFraudBlockError;
use crate::application::rewards::manage_reward_policy::RewardPolicyError;
use crate::application::rewards::submit_candidate::RewardCandidateSubmissionError;
use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
use serde_json::Value;

#[actix_web::test]
async fn candidate_permission_denied_uses_api_error_envelope() {
    let body = parse_body(
        reward_candidate_submission_error(RewardCandidateSubmissionError::PermissionDenied(
            "submit_reward_candidate".to_string(),
        ))
        .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::FORBIDDEN);
    assert_eq!(body.value["error"]["code"], "permission_denied");
    assert_eq!(
        body.value["error"]["message"],
        "User does not have reward candidate permission"
    );
}

#[actix_web::test]
async fn candidate_invalid_status_uses_conflict_envelope() {
    let body = parse_body(
        reward_candidate_submission_error(RewardCandidateSubmissionError::InvalidStatus(
            "cannot submit twice".to_string(),
        ))
        .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::CONFLICT);
    assert_eq!(body.value["error"]["code"], "invalid_reward_status");
    assert_eq!(body.value["error"]["message"], "cannot submit twice");
}

#[actix_web::test]
async fn policy_not_found_uses_specific_code() {
    let body = parse_body(reward_policy_error(RewardPolicyError::NotFound).error_response()).await;

    assert_eq!(body.status, StatusCode::NOT_FOUND);
    assert_eq!(body.value["error"]["code"], "reward_policy_not_found");
    assert_eq!(body.value["error"]["message"], "Reward policy not found");
}

#[actix_web::test]
async fn fraud_block_not_found_uses_specific_code() {
    let body =
        parse_body(reward_fraud_block_error(RewardFraudBlockError::NotFound).error_response())
            .await;

    assert_eq!(body.status, StatusCode::NOT_FOUND);
    assert_eq!(body.value["error"]["code"], "reward_fraud_block_not_found");
    assert_eq!(
        body.value["error"]["message"],
        "Reward fraud block not found"
    );
}

struct ParsedErrorBody {
    status: StatusCode,
    value: Value,
}

async fn parse_body(response: actix_web::HttpResponse) -> ParsedErrorBody {
    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let value = serde_json::from_slice(&body).unwrap();
    ParsedErrorBody { status, value }
}
