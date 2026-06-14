use bigdecimal::BigDecimal;

use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationRowOutput;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PlatformWalletReconciliationRowFact {
    pub wallet_id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub balance: BigDecimal,
    pub internal_transaction_count: i64,
    pub external_transaction_count: i64,
    pub reward_record_count: i64,
    pub needs_reconciliation_count: i64,
    pub missing_credit_count: i64,
    pub missing_notification_count: i64,
    pub missing_payout_count: i64,
}

pub(crate) fn platform_wallet_reconciliation_row(
    fact: PlatformWalletReconciliationRowFact,
) -> PlatformWalletReconciliationRowOutput {
    PlatformWalletReconciliationRowOutput {
        wallet_id: fact.wallet_id,
        owner_type: if fact.user_id.is_some() {
            "user".to_string()
        } else {
            "organization".to_string()
        },
        user_id: fact.user_id,
        organization_id: fact.organization_id,
        balance: fact.balance.to_string(),
        internal_transaction_count: fact.internal_transaction_count,
        external_transaction_count: fact.external_transaction_count,
        reward_record_count: fact.reward_record_count,
        needs_reconciliation_count: fact.needs_reconciliation_count,
        missing_credit_count: fact.missing_credit_count,
        missing_notification_count: fact.missing_notification_count,
        missing_payout_count: fact.missing_payout_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_user_wallet_reconciliation_row() {
        let row = platform_wallet_reconciliation_row(row_fact(Some(7), None));

        assert_eq!(row.owner_type, "user");
        assert_eq!(row.user_id, Some(7));
        assert_eq!(row.organization_id, None);
        assert_eq!(row.balance, "50");
    }

    #[test]
    fn builds_organization_wallet_reconciliation_row() {
        let row = platform_wallet_reconciliation_row(row_fact(None, Some(8)));

        assert_eq!(row.owner_type, "organization");
        assert_eq!(row.user_id, None);
        assert_eq!(row.organization_id, Some(8));
    }

    fn row_fact(
        user_id: Option<i32>,
        organization_id: Option<i32>,
    ) -> PlatformWalletReconciliationRowFact {
        PlatformWalletReconciliationRowFact {
            wallet_id: 1,
            user_id,
            organization_id,
            balance: BigDecimal::from(50),
            internal_transaction_count: 2,
            external_transaction_count: 3,
            reward_record_count: 4,
            needs_reconciliation_count: 5,
            missing_credit_count: 6,
            missing_notification_count: 7,
            missing_payout_count: 8,
        }
    }
}
