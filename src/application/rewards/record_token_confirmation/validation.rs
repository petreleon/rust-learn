use bigdecimal::BigDecimal;

use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmationCommand, RewardTokenConfirmationError,
};
use crate::domain::rewards::token::transaction_type_for_token_event;

pub fn validate_token_confirmation_command(
    command: &RewardTokenConfirmationCommand,
) -> Result<String, RewardTokenConfirmationError> {
    if command.chain_id <= 0 {
        return Err(RewardTokenConfirmationError::InvalidInput(
            "chain_id must be positive".to_string(),
        ));
    }
    if command.contract_address.trim().is_empty() {
        return Err(RewardTokenConfirmationError::InvalidInput(
            "contract_address is required".to_string(),
        ));
    }
    if command.transaction_hash.trim().is_empty() {
        return Err(RewardTokenConfirmationError::InvalidInput(
            "transaction_hash is required".to_string(),
        ));
    }
    if command.log_index < 0 {
        return Err(RewardTokenConfirmationError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    if command.to_address.trim().is_empty() {
        return Err(RewardTokenConfirmationError::InvalidInput(
            "to_address is required".to_string(),
        ));
    }
    if command.amount <= BigDecimal::from(0) {
        return Err(RewardTokenConfirmationError::InvalidInput(
            "amount must be positive".to_string(),
        ));
    }
    transaction_type_for_token_event(&command.event_type)
        .map(str::to_string)
        .ok_or_else(|| {
            RewardTokenConfirmationError::InvalidInput("unsupported token event type".to_string())
        })
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::validate_token_confirmation_command;
    use crate::application::rewards::record_token_confirmation::{
        RewardTokenConfirmationCommand, RewardTokenConfirmationError,
    };

    fn valid_command() -> RewardTokenConfirmationCommand {
        RewardTokenConfirmationCommand {
            chain_id: 1,
            contract_address: "0x1234".into(),
            transaction_hash: "0xabc".into(),
            log_index: 0,
            event_type: "transfer".into(),
            from_address: Some("0xfrom".into()),
            to_address: "0xto".into(),
            amount: BigDecimal::from(50),
        }
    }

    #[test]
    fn valid_command_returns_transaction_type() {
        assert_eq!(
            validate_token_confirmation_command(&valid_command()).unwrap(),
            "token_transfer"
        );
    }

    #[test]
    fn rejects_invalid_scalar_fields() {
        let mut command = valid_command();
        command.chain_id = 0;
        assert!(validate_token_confirmation_command(&command).is_err());

        command = valid_command();
        command.log_index = -1;
        assert!(validate_token_confirmation_command(&command).is_err());

        command = valid_command();
        command.amount = BigDecimal::from(0);
        assert!(validate_token_confirmation_command(&command).is_err());
    }

    #[test]
    fn rejects_missing_addresses_and_hash() {
        let mut command = valid_command();
        command.contract_address = "   ".into();
        assert!(validate_token_confirmation_command(&command).is_err());

        command = valid_command();
        command.transaction_hash = "".into();
        assert!(validate_token_confirmation_command(&command).is_err());

        command = valid_command();
        command.to_address = "".into();
        assert!(validate_token_confirmation_command(&command).is_err());
    }

    #[test]
    fn rejects_unsupported_token_event() {
        let mut command = valid_command();
        command.event_type = "burn".into();
        assert_eq!(
            validate_token_confirmation_command(&command).unwrap_err(),
            RewardTokenConfirmationError::InvalidInput("unsupported token event type".to_string())
        );
    }
}
