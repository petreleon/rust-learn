use crate::domain::rewards::candidate::event_type::RewardEventType;

pub type RewardEvidence = serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum RewardEvidenceError {
    MissingNumber { key: &'static str },
    BelowThreshold { key: &'static str },
}

pub fn ensure_reward_evidence_is_eligible(
    event_type: RewardEventType,
    evidence: &RewardEvidence,
) -> Result<(), RewardEvidenceError> {
    match event_type {
        RewardEventType::CourseCompletion => {
            ensure_evidence_number_at_least(evidence, "completion_percentage", 100.0)
        }
        RewardEventType::AssessmentCompletion => {
            ensure_evidence_number_at_least(evidence, "passing_score", 70.0)
        }
        RewardEventType::ManualCompletion | RewardEventType::AdministrativeAdjustment => Ok(()),
    }
}

fn ensure_evidence_number_at_least(
    evidence: &RewardEvidence,
    key: &'static str,
    minimum: f64,
) -> Result<(), RewardEvidenceError> {
    let value = evidence
        .get(key)
        .and_then(RewardEvidence::as_f64)
        .ok_or(RewardEvidenceError::MissingNumber { key })?;

    if value >= minimum {
        Ok(())
    } else {
        Err(RewardEvidenceError::BelowThreshold { key })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ensure_reward_evidence_is_eligible, RewardEvidenceError};
    use crate::domain::rewards::candidate::event_type::RewardEventType;

    #[test]
    fn validates_course_completion_threshold() {
        assert!(ensure_reward_evidence_is_eligible(
            RewardEventType::CourseCompletion,
            &json!({"completion_percentage": 100})
        )
        .is_ok());
        assert_eq!(
            ensure_reward_evidence_is_eligible(
                RewardEventType::CourseCompletion,
                &json!({"completion_percentage": 99})
            )
            .unwrap_err(),
            RewardEvidenceError::BelowThreshold {
                key: "completion_percentage"
            }
        );
    }

    #[test]
    fn validates_assessment_completion_threshold() {
        assert!(ensure_reward_evidence_is_eligible(
            RewardEventType::AssessmentCompletion,
            &json!({"passing_score": 70})
        )
        .is_ok());
        assert_eq!(
            ensure_reward_evidence_is_eligible(RewardEventType::AssessmentCompletion, &json!({}))
                .unwrap_err(),
            RewardEvidenceError::MissingNumber {
                key: "passing_score"
            }
        );
    }

    #[test]
    fn manual_events_need_no_evidence() {
        assert!(
            ensure_reward_evidence_is_eligible(RewardEventType::ManualCompletion, &json!({}))
                .is_ok()
        );
    }
}
