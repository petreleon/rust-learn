use bigdecimal::BigDecimal;

use crate::application::rewards::plan_payout::{RewardPayoutCandidate, RewardPayoutPlanError};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub fn approved_positive_amount(
    candidate: &RewardPayoutCandidate,
) -> Result<BigDecimal, RewardPayoutPlanError> {
    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardPayoutPlanError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardPayoutPlanError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }
    Ok(amount)
}

pub fn ensure_candidate_ready_for_payout(
    candidate: &RewardPayoutCandidate,
) -> Result<(), RewardPayoutPlanError> {
    if candidate.status == RewardCandidateStatus::AmountApproved {
        Ok(())
    } else {
        Err(RewardPayoutPlanError::InvalidStatus(
            "reward candidate must be amount approved before payout planning".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::{approved_positive_amount, ensure_candidate_ready_for_payout};
    use crate::application::rewards::plan_payout::{RewardPayoutCandidate, RewardPayoutPlanError};
    use crate::domain::rewards::candidate::event_type::RewardEventType;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;

    #[test]
    fn amount_approved_candidate_with_positive_amount_is_ready() {
        let candidate = candidate(
            RewardCandidateStatus::AmountApproved,
            Some(BigDecimal::from(10)),
        );

        ensure_candidate_ready_for_payout(&candidate).unwrap();
        assert_eq!(
            approved_positive_amount(&candidate).unwrap(),
            BigDecimal::from(10)
        );
    }

    #[test]
    fn non_amount_approved_candidate_is_not_ready() {
        assert_eq!(
            ensure_candidate_ready_for_payout(&candidate(
                RewardCandidateStatus::TeacherApproved,
                None
            ))
            .unwrap_err(),
            RewardPayoutPlanError::InvalidStatus(
                "reward candidate must be amount approved before payout planning".to_string()
            )
        );
    }

    #[test]
    fn missing_or_non_positive_amount_fails() {
        assert_eq!(
            approved_positive_amount(&candidate(RewardCandidateStatus::AmountApproved, None))
                .unwrap_err(),
            RewardPayoutPlanError::InvalidInput(
                "reward candidate must have an approved amount".to_string()
            )
        );
        assert_eq!(
            approved_positive_amount(&candidate(
                RewardCandidateStatus::AmountApproved,
                Some(BigDecimal::from(0))
            ))
            .unwrap_err(),
            RewardPayoutPlanError::InvalidInput(
                "approved reward amount must be positive".to_string()
            )
        );
    }

    fn candidate(
        status: RewardCandidateStatus,
        approved_amount: Option<BigDecimal>,
    ) -> RewardPayoutCandidate {
        RewardPayoutCandidate {
            id: 1,
            course_id: 2,
            event_type: RewardEventType::CourseCompletion,
            status,
            approved_amount,
        }
    }
}
