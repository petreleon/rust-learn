use crate::application::rewards::decide_amount::RewardAmountDecisionError;
use crate::application::rewards::decide_teacher_candidate::TeacherRewardCandidateDecisionError;
use crate::application::rewards::list_candidate_audit::RewardCandidateAuditError;
use crate::application::rewards::list_course_candidates::CourseRewardCandidatesError;
use crate::application::rewards::list_platform_candidates::PlatformRewardCandidatesError;
use crate::application::rewards::submit_candidate::RewardCandidateSubmissionError;
use crate::http::errors::ApiError;

const CANDIDATE_PERMISSION: &str = "User does not have reward candidate permission";
const CANDIDATE_FAILURE: &str = "Failed to process reward candidate";

pub(in crate::http::rewards) fn reward_candidate_submission_error(
    error: RewardCandidateSubmissionError,
) -> ApiError {
    match error {
        RewardCandidateSubmissionError::PermissionDenied(_) => {
            super::permission_denied(CANDIDATE_PERMISSION)
        }
        RewardCandidateSubmissionError::InvalidInput(message) => super::invalid_input(message),
        RewardCandidateSubmissionError::InvalidStatus(message) => super::invalid_status(message),
        RewardCandidateSubmissionError::NotFound => super::candidate_not_found(),
        RewardCandidateSubmissionError::Connection(_) => super::db_connection_failed(),
        RewardCandidateSubmissionError::Database(message) => {
            super::logged_internal("reward_candidate_api_failed", CANDIDATE_FAILURE, message)
        }
    }
}

pub(in crate::http::rewards) fn teacher_decision_error(
    error: TeacherRewardCandidateDecisionError,
) -> ApiError {
    match error {
        TeacherRewardCandidateDecisionError::PermissionDenied(_) => {
            super::permission_denied(CANDIDATE_PERMISSION)
        }
        TeacherRewardCandidateDecisionError::InvalidStatus(message) => {
            super::invalid_status(message)
        }
        TeacherRewardCandidateDecisionError::NotFound => super::candidate_not_found(),
        TeacherRewardCandidateDecisionError::Connection(_) => super::db_connection_failed(),
        TeacherRewardCandidateDecisionError::Database(message) => {
            super::logged_internal("reward_candidate_api_failed", CANDIDATE_FAILURE, message)
        }
    }
}

pub(in crate::http::rewards) fn amount_decision_error(
    error: RewardAmountDecisionError,
) -> ApiError {
    match error {
        RewardAmountDecisionError::PermissionDenied(_) => {
            super::permission_denied(CANDIDATE_PERMISSION)
        }
        RewardAmountDecisionError::InvalidInput(message) => super::invalid_input(message),
        RewardAmountDecisionError::InvalidStatus(message) => super::invalid_status(message),
        RewardAmountDecisionError::NotFound => super::candidate_not_found(),
        RewardAmountDecisionError::Connection(_) => super::db_connection_failed(),
        RewardAmountDecisionError::Database(message) => {
            super::logged_internal("reward_candidate_api_failed", CANDIDATE_FAILURE, message)
        }
    }
}

pub(in crate::http::rewards) fn platform_reward_candidates_error(
    error: PlatformRewardCandidatesError,
) -> ApiError {
    match error {
        PlatformRewardCandidatesError::PermissionDenied(_) => {
            super::permission_denied(CANDIDATE_PERMISSION)
        }
        PlatformRewardCandidatesError::InvalidStatus(message) => super::invalid_status(message),
        PlatformRewardCandidatesError::Connection(_) => super::db_connection_failed(),
        PlatformRewardCandidatesError::Database(message) => super::logged_internal(
            "platform_reward_candidates_api_failed",
            CANDIDATE_FAILURE,
            message,
        ),
    }
}

pub(in crate::http::rewards) fn course_reward_candidates_error(
    error: CourseRewardCandidatesError,
) -> ApiError {
    match error {
        CourseRewardCandidatesError::PermissionDenied(_) => {
            super::permission_denied(CANDIDATE_PERMISSION)
        }
        CourseRewardCandidatesError::InvalidStatus(message) => super::invalid_status(message),
        CourseRewardCandidatesError::NotFound => super::candidate_not_found(),
        CourseRewardCandidatesError::Connection(_) => super::db_connection_failed(),
        CourseRewardCandidatesError::Database(message) => super::logged_internal(
            "course_reward_candidates_api_failed",
            CANDIDATE_FAILURE,
            message,
        ),
    }
}

pub(in crate::http::rewards) fn reward_candidate_audit_error(
    error: RewardCandidateAuditError,
) -> ApiError {
    match error {
        RewardCandidateAuditError::PermissionDenied(_) => {
            super::permission_denied(CANDIDATE_PERMISSION)
        }
        RewardCandidateAuditError::NotFound => super::candidate_not_found(),
        RewardCandidateAuditError::Connection(_) => super::db_connection_failed(),
        RewardCandidateAuditError::Database(message) => super::logged_internal(
            "reward_candidate_audit_api_failed",
            CANDIDATE_FAILURE,
            message,
        ),
    }
}
