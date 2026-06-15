use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::application::reporting::platform_csv_exports::PlatformTokenPayoutExportRowOutput;
use crate::domain::rewards::token::RewardTokenEventType;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PlatformTokenPayoutExportFact {
    pub reward_payout_record_id: i64,
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub payout_transaction_id: i64,
    pub external_transaction_id: i64,
    pub amount: BigDecimal,
    pub blockchain_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<RewardTokenEventType>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub(crate) fn platform_token_payout_export_row(
    fact: PlatformTokenPayoutExportFact,
) -> PlatformTokenPayoutExportRowOutput {
    PlatformTokenPayoutExportRowOutput {
        reward_payout_record_id: fact.reward_payout_record_id,
        reward_candidate_id: fact.reward_candidate_id,
        course_id: fact.course_id,
        student_user_id: fact.student_user_id,
        payout_transaction_id: fact.payout_transaction_id,
        external_transaction_id: fact.external_transaction_id,
        amount: fact.amount.to_string(),
        blockchain_address: fact.blockchain_address,
        chain_id: fact.chain_id,
        contract_address: fact.contract_address.unwrap_or_default(),
        transaction_hash: fact.transaction_hash.unwrap_or_default(),
        log_index: fact.log_index,
        event_type: fact.event_type,
        from_address: fact.from_address.unwrap_or_default(),
        to_address: fact.to_address.unwrap_or_default(),
        created_at: fact.created_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_token_payout_export_row_from_fact() {
        let now = Utc::now();

        let row = platform_token_payout_export_row(PlatformTokenPayoutExportFact {
            reward_payout_record_id: 7,
            reward_candidate_id: 8,
            course_id: 9,
            student_user_id: 10,
            payout_transaction_id: 11,
            external_transaction_id: 12,
            amount: BigDecimal::from(25),
            blockchain_address: "0xchain".to_string(),
            chain_id: Some(31337),
            contract_address: Some("0xcontract".to_string()),
            transaction_hash: Some("0xtx".to_string()),
            log_index: Some(2),
            event_type: Some(RewardTokenEventType::Transfer),
            from_address: Some("0xfrom".to_string()),
            to_address: Some("0xto".to_string()),
            created_at: now,
        });

        assert_eq!(row.reward_payout_record_id, 7);
        assert_eq!(row.reward_candidate_id, 8);
        assert_eq!(row.amount, "25");
        assert_eq!(row.contract_address, "0xcontract");
        assert_eq!(row.transaction_hash, "0xtx");
        assert_eq!(row.event_type, Some(RewardTokenEventType::Transfer));
        assert_eq!(row.from_address, "0xfrom");
        assert_eq!(row.to_address, "0xto");
    }

    #[test]
    fn defaults_missing_optional_external_transaction_text() {
        let now = Utc::now();

        let row = platform_token_payout_export_row(PlatformTokenPayoutExportFact {
            reward_payout_record_id: 1,
            reward_candidate_id: 2,
            course_id: 3,
            student_user_id: 4,
            payout_transaction_id: 5,
            external_transaction_id: 6,
            amount: BigDecimal::from(0),
            blockchain_address: "0xchain".to_string(),
            chain_id: None,
            contract_address: None,
            transaction_hash: None,
            log_index: None,
            event_type: None,
            from_address: None,
            to_address: None,
            created_at: now,
        });

        assert_eq!(row.contract_address, "");
        assert_eq!(row.transaction_hash, "");
        assert_eq!(row.event_type, None);
        assert_eq!(row.from_address, "");
        assert_eq!(row.to_address, "");
    }
}
