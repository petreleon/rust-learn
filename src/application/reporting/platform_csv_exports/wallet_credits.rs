use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::application::reporting::platform_csv_exports::PlatformWalletCreditExportRowOutput;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PlatformWalletCreditExportFact {
    pub reward_wallet_credit_record_id: i64,
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: BigDecimal,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub(crate) fn platform_wallet_credit_export_row(
    fact: PlatformWalletCreditExportFact,
) -> PlatformWalletCreditExportRowOutput {
    PlatformWalletCreditExportRowOutput {
        reward_wallet_credit_record_id: fact.reward_wallet_credit_record_id,
        reward_candidate_id: fact.reward_candidate_id,
        course_id: fact.course_id,
        student_user_id: fact.student_user_id,
        wallet_id: fact.wallet_id,
        transaction_id: fact.transaction_id,
        internal_transaction_id: fact.internal_transaction_id,
        amount: fact.amount.to_string(),
        notification_id: fact.notification_id,
        notified_at: fact.notified_at,
        created_at: fact.created_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_wallet_credit_export_row_from_fact() {
        let now = Utc::now();

        let row = platform_wallet_credit_export_row(PlatformWalletCreditExportFact {
            reward_wallet_credit_record_id: 7,
            reward_candidate_id: 8,
            course_id: 9,
            student_user_id: 10,
            wallet_id: 11,
            transaction_id: 12,
            internal_transaction_id: 13,
            amount: BigDecimal::from(25),
            notification_id: Some(14),
            notified_at: Some(now),
            created_at: now,
        });

        assert_eq!(row.reward_wallet_credit_record_id, 7);
        assert_eq!(row.reward_candidate_id, 8);
        assert_eq!(row.amount, "25");
        assert_eq!(row.notification_id, Some(14));
        assert_eq!(row.notified_at, Some(now));
    }
}
