use bigdecimal::BigDecimal;

use crate::application::wallet::burn_tokens::{TokenBurnCommand, TokenBurnError};
use crate::domain::wallet::burn::{TokenBurnFeePath, TokenBurnSource, TokenBurnStatus};

pub(super) fn validate_command(command: &TokenBurnCommand) -> Result<(), TokenBurnError> {
    if command.amount <= BigDecimal::from(0) {
        return Err(TokenBurnError::InvalidInput(
            "amount must be positive".to_string(),
        ));
    }
    if command.idempotency_key.trim().is_empty() {
        return Err(TokenBurnError::InvalidInput(
            "idempotency_key is required".to_string(),
        ));
    }
    validate_optional_text(command.ethereum_address.as_ref(), "ethereum_address")?;
    validate_optional_text(command.platform_address.as_ref(), "platform_address")?;
    validate_optional_text(command.contract_address.as_ref(), "contract_address")?;
    validate_optional_text(command.transaction_hash.as_ref(), "transaction_hash")?;
    if command.chain_id.map(|value| value <= 0).unwrap_or(false) {
        return Err(TokenBurnError::InvalidInput(
            "chain_id must be positive when provided".to_string(),
        ));
    }
    if command.log_index.map(|value| value < 0).unwrap_or(false) {
        return Err(TokenBurnError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    Ok(())
}

pub(super) fn ensure_fee_path_matches_source(
    source: TokenBurnSource,
    fee_path: TokenBurnFeePath,
) -> Result<(), TokenBurnError> {
    let valid = matches!(
        (source, fee_path),
        (TokenBurnSource::CentralizedWallet, TokenBurnFeePath::None)
            | (
                TokenBurnSource::CentralizedWallet,
                TokenBurnFeePath::PlatformSubsidized
            )
            | (
                TokenBurnSource::DecentralizedDirect,
                TokenBurnFeePath::NetworkFeePaidByUser
            )
            | (
                TokenBurnSource::DecentralizedPlatformMediated,
                TokenBurnFeePath::PlatformDepositFee
            )
    );
    if valid {
        Ok(())
    } else {
        Err(TokenBurnError::InvalidInput(
            "fee_path is not valid for the selected burn source".to_string(),
        ))
    }
}

pub(super) fn status_for_source(
    command: &TokenBurnCommand,
    source: TokenBurnSource,
) -> TokenBurnStatus {
    match source {
        TokenBurnSource::DecentralizedPlatformMediated
            if command.transaction_hash.is_none() && command.deposit_intent_id.is_some() =>
        {
            TokenBurnStatus::DepositPending
        }
        _ => TokenBurnStatus::LeaderboardIndexed,
    }
}

fn validate_optional_text(value: Option<&String>, field: &str) -> Result<(), TokenBurnError> {
    if value.map(|value| value.trim().is_empty()).unwrap_or(false) {
        return Err(TokenBurnError::InvalidInput(format!(
            "{} cannot be empty when provided",
            field
        )));
    }
    Ok(())
}
