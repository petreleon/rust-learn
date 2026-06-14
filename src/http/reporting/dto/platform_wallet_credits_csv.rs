use crate::application::reporting::platform_csv_exports::PlatformWalletCreditExportRowOutput;
use crate::http::reporting::dto::{csv::csv_optional, csv::csv_value};

pub fn platform_wallet_credits_csv(rows: &[PlatformWalletCreditExportRowOutput]) -> String {
    let mut csv = String::from(
        "reward_wallet_credit_record_id,reward_candidate_id,course_id,student_user_id,wallet_id,transaction_id,internal_transaction_id,amount,notification_id,notified_at,created_at\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            row.reward_wallet_credit_record_id,
            row.reward_candidate_id,
            row.course_id,
            row.student_user_id,
            row.wallet_id,
            row.transaction_id,
            row.internal_transaction_id,
            csv_value(&row.amount),
            csv_optional(row.notification_id),
            csv_optional(row.notified_at.as_ref()),
            row.created_at
        ));
    }
    csv
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::platform_wallet_credits_csv;
    use crate::application::reporting::platform_csv_exports::PlatformWalletCreditExportRowOutput;

    #[test]
    fn keeps_legacy_wallet_credits_csv_shape() {
        let csv = platform_wallet_credits_csv(&[PlatformWalletCreditExportRowOutput {
            reward_wallet_credit_record_id: 1,
            reward_candidate_id: 2,
            course_id: 3,
            student_user_id: 4,
            wallet_id: 5,
            transaction_id: 6,
            internal_transaction_id: 7,
            amount: "10".to_string(),
            notification_id: None,
            notified_at: None,
            created_at: Utc::now(),
        }]);

        assert!(csv.starts_with("reward_wallet_credit_record_id,reward_candidate_id"));
        assert!(csv.contains("1,2,3,4,5,6,7,10"));
    }
}
