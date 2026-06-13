use crate::application::rewards::decide_teacher_candidate::TeacherRewardCandidateDecisionError;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub(super) fn normalize_teacher_decision_status(
    status: &str,
) -> Result<RewardCandidateStatus, TeacherRewardCandidateDecisionError> {
    match status.trim().to_ascii_lowercase().as_str() {
        "approved" | "teacher_approved" => Ok(RewardCandidateStatus::TeacherApproved),
        "rejected" | "teacher_rejected" => Ok(RewardCandidateStatus::TeacherRejected),
        _ => Err(TeacherRewardCandidateDecisionError::InvalidStatus(
            "unsupported teacher reward decision status".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_teacher_decision_status;
    use crate::application::rewards::decide_teacher_candidate::TeacherRewardCandidateDecisionError;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;

    #[test]
    fn normalizes_legacy_teacher_decision_aliases() {
        assert_eq!(
            normalize_teacher_decision_status(" approved ").unwrap(),
            RewardCandidateStatus::TeacherApproved
        );
        assert_eq!(
            normalize_teacher_decision_status("teacher_rejected").unwrap(),
            RewardCandidateStatus::TeacherRejected
        );
    }

    #[test]
    fn rejects_unsupported_teacher_decision_status() {
        assert_eq!(
            normalize_teacher_decision_status("teacher-approved").unwrap_err(),
            TeacherRewardCandidateDecisionError::InvalidStatus(
                "unsupported teacher reward decision status".to_string()
            )
        );
    }
}
