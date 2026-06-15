use crate::application::rewards::submit_candidate::RewardCandidateSubmissionError;
use crate::domain::rewards::candidate::event_type::RewardEventType;

pub(super) fn normalize_idempotency_key(
    idempotency_key: Option<String>,
    course_id: i32,
    student_user_id: i32,
    event_type: RewardEventType,
) -> Result<String, RewardCandidateSubmissionError> {
    match idempotency_key {
        Some(key) => {
            let trimmed = key.trim();
            if trimmed.is_empty() {
                Err(RewardCandidateSubmissionError::InvalidInput(
                    "idempotency key cannot be blank".to_string(),
                ))
            } else {
                Ok(trimmed.to_string())
            }
        }
        None => Ok(format!(
            "{}:{}:{}:manual",
            event_type.as_str(),
            course_id,
            student_user_id
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_idempotency_key;
    use crate::domain::rewards::candidate::event_type::RewardEventType;

    #[test]
    fn trims_or_generates_idempotency_key() {
        assert_eq!(
            normalize_idempotency_key(
                Some(" key ".to_string()),
                1,
                2,
                RewardEventType::ManualCompletion
            )
            .unwrap(),
            "key"
        );
        assert_eq!(
            normalize_idempotency_key(None, 1, 2, RewardEventType::ManualCompletion).unwrap(),
            "manual_completion:1:2:manual"
        );
    }
}
