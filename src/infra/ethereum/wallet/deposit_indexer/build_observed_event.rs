use crate::application::wallet::index_deposit::ObservedWalletDepositEvent;
use crate::domain::wallet::deposit::WalletDepositEventType;
use bigdecimal::BigDecimal;
use ethers::types::{Address, Log, U256};
use std::str::FromStr;

pub(super) fn build_observed_event(
    log: Log,
    contract_address: Address,
    event_type: WalletDepositEventType,
    from_address: Address,
    to_address: Address,
    chain_id: i64,
    token_decimals: u32,
) -> Result<ObservedWalletDepositEvent, String> {
    let transaction_hash = log
        .transaction_hash
        .ok_or_else(|| "log is missing transaction_hash".to_string())?;
    let log_index = log
        .log_index
        .ok_or_else(|| "log is missing log_index".to_string())?
        .as_u64() as i64;
    let amount = amount_from_log_data(log.data.0.as_ref(), token_decimals)?;

    Ok(ObservedWalletDepositEvent {
        chain_id,
        contract_address: format!("{:#x}", contract_address),
        transaction_hash: format!("{:#x}", transaction_hash),
        log_index,
        event_type,
        from_address: format!("{:#x}", from_address),
        to_address: format!("{:#x}", to_address),
        amount,
    })
}

fn amount_from_log_data(data: &[u8], token_decimals: u32) -> Result<BigDecimal, String> {
    if data.len() != 32 {
        return Err(format!(
            "event amount data must be 32 bytes, got {}",
            data.len()
        ));
    }
    let amount = U256::from_big_endian(data);
    u256_to_decimal(amount, token_decimals)
}

fn u256_to_decimal(amount: U256, token_decimals: u32) -> Result<BigDecimal, String> {
    let raw = amount.to_string();
    if token_decimals == 0 {
        return BigDecimal::from_str(&raw)
            .map_err(|error| format!("failed to parse token amount: {error}"));
    }

    let decimals = token_decimals as usize;
    let decimal = if raw.len() <= decimals {
        format!("0.{}{}", "0".repeat(decimals - raw.len()), raw)
    } else {
        let split = raw.len() - decimals;
        format!("{}.{}", &raw[..split], &raw[split..])
    };
    let trimmed = decimal.trim_end_matches('0').trim_end_matches('.');
    BigDecimal::from_str(if trimmed.is_empty() { "0" } else { trimmed })
        .map_err(|error| format!("failed to parse token amount: {error}"))
}
