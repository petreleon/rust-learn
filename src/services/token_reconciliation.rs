use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::transaction::{
    ExternalTransaction, NewExternalTransaction, Transaction, TransactionLink,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenEventKind {
    Mint,
    Transfer,
    Import,
}

impl TokenEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenEventKind::Mint => "mint",
            TokenEventKind::Transfer => "transfer",
            TokenEventKind::Import => "import",
        }
    }

    pub fn transaction_type(self) -> &'static str {
        match self {
            TokenEventKind::Mint => "token_mint",
            TokenEventKind::Transfer => "token_transfer",
            TokenEventKind::Import => "token_import",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObservedTokenEvent {
    pub chain_id: i64,
    pub contract_address: String,
    pub transaction_hash: String,
    pub log_index: i64,
    pub event_type: TokenEventKind,
    pub from_address: Option<String>,
    pub to_address: String,
    pub amount: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenReconciliationRecord {
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub inserted: bool,
}

fn validate_observed_event(event: &ObservedTokenEvent) -> Result<()> {
    if event.chain_id <= 0 {
        return Err(anyhow!("chain_id must be positive"));
    }
    if event.contract_address.trim().is_empty() {
        return Err(anyhow!("contract_address is required"));
    }
    if event.transaction_hash.trim().is_empty() {
        return Err(anyhow!("transaction_hash is required"));
    }
    if event.log_index < 0 {
        return Err(anyhow!("log_index cannot be negative"));
    }
    if event.to_address.trim().is_empty() {
        return Err(anyhow!("to_address is required"));
    }
    if event.amount <= BigDecimal::from(0) {
        return Err(anyhow!("amount must be positive"));
    }

    Ok(())
}

pub fn record_token_event(
    conn: &mut PgConnection,
    event: &ObservedTokenEvent,
) -> Result<TokenReconciliationRecord> {
    validate_observed_event(event)?;

    conn.transaction::<TokenReconciliationRecord, anyhow::Error, _>(|tx| {
        if let Some(existing) = ExternalTransaction::find_by_chain_tx_log(
            event.chain_id,
            &event.transaction_hash,
            event.log_index,
            tx,
        )? {
            let transaction_id =
                match TransactionLink::find_transaction_for_external(existing.id, tx)? {
                    Some(transaction_id) => transaction_id,
                    None => {
                        let transaction_id =
                            Transaction::create(event.event_type.transaction_type(), tx)?;
                        TransactionLink::create_external(transaction_id, existing.id, tx)?;
                        transaction_id
                    }
                };

            return Ok(TokenReconciliationRecord {
                transaction_id,
                external_transaction_id: existing.id,
                inserted: false,
            });
        }

        let transaction_id = Transaction::create(event.event_type.transaction_type(), tx)?;
        let external_transaction_id = ExternalTransaction::create(
            NewExternalTransaction {
                amount: event.amount.clone(),
                blockchain_address: &event.to_address,
                chain_id: Some(event.chain_id),
                contract_address: Some(&event.contract_address),
                transaction_hash: Some(&event.transaction_hash),
                log_index: Some(event.log_index),
                event_type: Some(event.event_type.as_str()),
                from_address: event.from_address.as_deref(),
                to_address: Some(&event.to_address),
            },
            tx,
        )?;
        TransactionLink::create_external(transaction_id, external_transaction_id, tx)?;

        Ok(TokenReconciliationRecord {
            transaction_id,
            external_transaction_id,
            inserted: true,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::{validate_observed_event, ObservedTokenEvent, TokenEventKind};
    use bigdecimal::BigDecimal;

    fn valid_event() -> ObservedTokenEvent {
        ObservedTokenEvent {
            chain_id: 31337,
            contract_address: "0x0000000000000000000000000000000000000001".to_string(),
            transaction_hash: "0xabc".to_string(),
            log_index: 0,
            event_type: TokenEventKind::Transfer,
            from_address: Some("0x0000000000000000000000000000000000000002".to_string()),
            to_address: "0x0000000000000000000000000000000000000003".to_string(),
            amount: BigDecimal::from(100),
        }
    }

    #[test]
    fn token_event_kind_maps_to_transaction_types() {
        assert_eq!(TokenEventKind::Mint.transaction_type(), "token_mint");
        assert_eq!(
            TokenEventKind::Transfer.transaction_type(),
            "token_transfer"
        );
        assert_eq!(TokenEventKind::Import.transaction_type(), "token_import");
    }

    #[test]
    fn validates_required_event_fields() {
        assert!(validate_observed_event(&valid_event()).is_ok());

        let mut missing_hash = valid_event();
        missing_hash.transaction_hash = " ".to_string();
        assert_eq!(
            validate_observed_event(&missing_hash)
                .unwrap_err()
                .to_string(),
            "transaction_hash is required"
        );

        let mut zero_amount = valid_event();
        zero_amount.amount = BigDecimal::from(0);
        assert_eq!(
            validate_observed_event(&zero_amount)
                .unwrap_err()
                .to_string(),
            "amount must be positive"
        );
    }
}
