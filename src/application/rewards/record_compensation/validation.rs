use bigdecimal::BigDecimal;

use crate::application::rewards::record_compensation::{
    RecordRewardCompensationCommand, RewardCompensationError,
};

pub fn validate_compensation_command(
    command: &RecordRewardCompensationCommand,
) -> Result<(), RewardCompensationError> {
    if command.amount == BigDecimal::from(0) {
        return Err(RewardCompensationError::InvalidInput(
            "compensation amount cannot be zero".to_string(),
        ));
    }
    if command.reason.trim().is_empty() {
        return Err(RewardCompensationError::InvalidInput(
            "compensation reason is required".to_string(),
        ));
    }
    if command.idempotency_key.trim().is_empty() {
        return Err(RewardCompensationError::InvalidInput(
            "compensation idempotency key is required".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_compensation_command;
    use crate::application::rewards::record_compensation::{
        RecordRewardCompensationCommand, RewardCompensationError,
    };
    use bigdecimal::BigDecimal;

    fn command(
        amount: i64,
        reason: &str,
        idempotency_key: &str,
    ) -> RecordRewardCompensationCommand {
        RecordRewardCompensationCommand {
            reward_candidate_id: 1,
            amount: BigDecimal::from(amount),
            reason: reason.to_string(),
            idempotency_key: idempotency_key.to_string(),
        }
    }

    #[test]
    fn valid_and_negative_amounts_pass() {
        validate_compensation_command(&command(100, "Adjustment", "key-1")).unwrap();
        validate_compensation_command(&command(-50, "Negative comp", "key-2")).unwrap();
    }

    #[test]
    fn zero_amount_fails() {
        assert_eq!(
            validate_compensation_command(&command(0, "reason", "key")).unwrap_err(),
            RewardCompensationError::InvalidInput("compensation amount cannot be zero".to_string())
        );
    }

    #[test]
    fn empty_reason_fails() {
        assert_eq!(
            validate_compensation_command(&command(100, "   ", "key")).unwrap_err(),
            RewardCompensationError::InvalidInput("compensation reason is required".to_string())
        );
    }

    #[test]
    fn empty_idempotency_key_fails() {
        assert_eq!(
            validate_compensation_command(&command(100, "reason", "")).unwrap_err(),
            RewardCompensationError::InvalidInput(
                "compensation idempotency key is required".to_string()
            )
        );
    }
}
