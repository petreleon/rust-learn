use super::lifecycle::{
    allows_new_submission_after_prior_candidate, prior_candidate_statuses_allowing_new_submission,
};
use super::status::RewardCandidateStatus as Status;

#[test]
fn new_submission_is_allowed_only_after_terminal_prior_candidate_states() {
    assert_eq!(
        prior_candidate_statuses_allowing_new_submission(),
        [
            Status::TeacherRejected,
            Status::AmountRejected,
            Status::Failed
        ]
    );
    assert!(allows_new_submission_after_prior_candidate(
        Status::TeacherRejected
    ));
    assert!(allows_new_submission_after_prior_candidate(
        Status::AmountRejected
    ));
    assert!(allows_new_submission_after_prior_candidate(Status::Failed));
    assert!(!allows_new_submission_after_prior_candidate(
        Status::PendingTeacherApproval
    ));
    assert!(!allows_new_submission_after_prior_candidate(
        Status::Completed
    ));
}
