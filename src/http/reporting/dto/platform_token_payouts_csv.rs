use crate::application::reporting::platform_csv_exports::PlatformTokenPayoutExportRowOutput;
use crate::http::reporting::dto::csv::{csv_optional, csv_value};

pub fn platform_token_payouts_csv(rows: &[PlatformTokenPayoutExportRowOutput]) -> String {
    let mut csv = String::from(
        "reward_payout_record_id,reward_candidate_id,course_id,student_user_id,payout_transaction_id,external_transaction_id,amount,blockchain_address,chain_id,contract_address,transaction_hash,log_index,event_type,from_address,to_address,created_at\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.reward_payout_record_id,
            row.reward_candidate_id,
            row.course_id,
            row.student_user_id,
            row.payout_transaction_id,
            row.external_transaction_id,
            csv_value(&row.amount),
            csv_value(&row.blockchain_address),
            csv_optional(row.chain_id),
            csv_value(&row.contract_address),
            csv_value(&row.transaction_hash),
            csv_optional(row.log_index),
            csv_value(&row.event_type),
            csv_value(&row.from_address),
            csv_value(&row.to_address),
            row.created_at
        ));
    }
    csv
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::platform_token_payouts_csv;
    use crate::application::reporting::platform_csv_exports::PlatformTokenPayoutExportRowOutput;

    #[test]
    fn keeps_legacy_token_payouts_csv_shape() {
        let csv = platform_token_payouts_csv(&[PlatformTokenPayoutExportRowOutput {
            reward_payout_record_id: 5,
            reward_candidate_id: 6,
            course_id: 7,
            student_user_id: 8,
            payout_transaction_id: 9,
            external_transaction_id: 10,
            amount: "12.5".to_string(),
            blockchain_address: "0xabc".to_string(),
            chain_id: Some(1),
            contract_address: "0xcontract".to_string(),
            transaction_hash: "0xtx".to_string(),
            log_index: Some(2),
            event_type: "transfer".to_string(),
            from_address: "from".to_string(),
            to_address: "to".to_string(),
            created_at: Utc::now(),
        }]);

        assert!(csv.starts_with("reward_payout_record_id,reward_candidate_id"));
        assert!(csv.contains("5,6,7,8,9,10,12.5"));
    }
}
