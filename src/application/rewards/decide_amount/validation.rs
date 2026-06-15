use bigdecimal::BigDecimal;

use crate::application::rewards::decide_amount::RewardAmountDecisionError;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::candidate::transition::amount_decision_target_status;

pub(super) fn normalize_amount_decision_status(
    status: &str,
) -> Result<RewardCandidateStatus, RewardAmountDecisionError> {
    amount_decision_target_status(status).ok_or_else(|| {
        RewardAmountDecisionError::InvalidStatus(
            "unsupported reward amount decision status".to_string(),
        )
    })
}

pub(super) fn approved_amount_for_status(
    target_status: RewardCandidateStatus,
    approved_amount: Option<BigDecimal>,
) -> Result<Option<BigDecimal>, RewardAmountDecisionError> {
    match target_status {
        RewardCandidateStatus::AmountApproved => {
            let amount = approved_amount.ok_or_else(|| {
                RewardAmountDecisionError::InvalidInput(
                    "approved amount is required for amount approval".to_string(),
                )
            })?;
            if amount < BigDecimal::from(0) {
                Err(RewardAmountDecisionError::InvalidInput(
                    "approved amount cannot be negative".to_string(),
                ))
            } else {
                Ok(Some(amount))
            }
        }
        RewardCandidateStatus::AmountRejected => Ok(None),
        _ => unreachable!("amount status normalization returned unsupported status"),
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::{approved_amount_for_status, normalize_amount_decision_status};
    use crate::application::rewards::decide_amount::RewardAmountDecisionError;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;

    #[test]
    fn normalizes_legacy_amount_decision_aliases() {
        assert_eq!(
            normalize_amount_decision_status(" APPROVED ").unwrap(),
            RewardCandidateStatus::AmountApproved
        );
        assert_eq!(
            normalize_amount_decision_status("amount_rejected").unwrap(),
            RewardCandidateStatus::AmountRejected
        );
    }

    #[test]
    fn rejects_unsupported_amount_status() {
        assert_eq!(
            normalize_amount_decision_status("pending").unwrap_err(),
            RewardAmountDecisionError::InvalidStatus(
                "unsupported reward amount decision status".to_string()
            )
        );
    }

    #[test]
    fn requires_non_negative_amount_for_approval() {
        assert_eq!(
            approved_amount_for_status(RewardCandidateStatus::AmountApproved, None).unwrap_err(),
            RewardAmountDecisionError::InvalidInput(
                "approved amount is required for amount approval".to_string()
            )
        );
        assert_eq!(
            approved_amount_for_status(
                RewardCandidateStatus::AmountApproved,
                Some(BigDecimal::from(-1))
            )
            .unwrap_err(),
            RewardAmountDecisionError::InvalidInput(
                "approved amount cannot be negative".to_string()
            )
        );
        assert_eq!(
            approved_amount_for_status(
                RewardCandidateStatus::AmountApproved,
                Some(BigDecimal::from(0))
            )
            .unwrap(),
            Some(BigDecimal::from(0))
        );
    }

    #[test]
    fn ignores_amount_for_rejection() {
        assert_eq!(
            approved_amount_for_status(
                RewardCandidateStatus::AmountRejected,
                Some(BigDecimal::from(99))
            )
            .unwrap(),
            None
        );
    }
}
