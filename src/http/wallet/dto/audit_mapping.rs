use crate::application::wallet::audit_wallet::{
    WalletAudit, WalletAuditWallet, WalletCompensationRecordAudit, WalletDepositIntentAudit,
    WalletExternalTransactionAudit, WalletInternalTransactionAudit, WalletRewardRecordAudit,
};
use crate::http::wallet::dto::audit::{
    WalletAuditResponse, WalletAuditWalletResponse, WalletCompensationRecordAuditResponse,
    WalletDepositIntentAuditResponse, WalletExternalTransactionAuditResponse,
    WalletInternalTransactionAuditResponse, WalletRewardRecordAuditResponse,
};

impl From<WalletAudit> for WalletAuditResponse {
    fn from(audit: WalletAudit) -> Self {
        Self {
            wallet: WalletAuditWalletResponse::from(audit.wallet),
            internal_transactions: audit
                .internal_transactions
                .into_iter()
                .map(WalletInternalTransactionAuditResponse::from)
                .collect(),
            external_transactions: audit
                .external_transactions
                .into_iter()
                .map(WalletExternalTransactionAuditResponse::from)
                .collect(),
            deposit_intents: audit
                .deposit_intents
                .into_iter()
                .map(WalletDepositIntentAuditResponse::from)
                .collect(),
            reward_records: audit
                .reward_records
                .into_iter()
                .map(WalletRewardRecordAuditResponse::from)
                .collect(),
            compensation_records: audit
                .compensation_records
                .into_iter()
                .map(WalletCompensationRecordAuditResponse::from)
                .collect(),
        }
    }
}

impl From<WalletAuditWallet> for WalletAuditWalletResponse {
    fn from(wallet: WalletAuditWallet) -> Self {
        Self {
            id: wallet.id,
            owner_type: wallet.owner_type.as_str().to_string(),
            user_id: wallet.user_id,
            organization_id: wallet.organization_id,
            value: wallet.value,
        }
    }
}

impl From<WalletInternalTransactionAudit> for WalletInternalTransactionAuditResponse {
    fn from(transaction: WalletInternalTransactionAudit) -> Self {
        Self {
            internal_transaction_id: transaction.internal_transaction_id,
            transaction_id: transaction.transaction_id,
            transaction_type: transaction.transaction_type,
            amount: transaction.amount,
            created_at: transaction.created_at,
        }
    }
}

impl From<WalletExternalTransactionAudit> for WalletExternalTransactionAuditResponse {
    fn from(transaction: WalletExternalTransactionAudit) -> Self {
        Self {
            external_transaction_id: transaction.external_transaction_id,
            transaction_id: transaction.transaction_id,
            reward_candidate_id: transaction.reward_candidate_id,
            amount: transaction.amount,
            blockchain_address: transaction.blockchain_address,
            chain_id: transaction.chain_id,
            contract_address: transaction.contract_address,
            transaction_hash: transaction.transaction_hash,
            log_index: transaction.log_index,
            event_type: transaction
                .event_type
                .map(|event_type| event_type.as_str().to_string()),
            from_address: transaction.from_address,
            to_address: transaction.to_address,
        }
    }
}

impl From<WalletDepositIntentAudit> for WalletDepositIntentAuditResponse {
    fn from(intent: WalletDepositIntentAudit) -> Self {
        Self {
            id: intent.id,
            user_id: intent.user_id,
            wallet_id: intent.wallet_id,
            ethereum_address: intent.ethereum_address,
            platform_address: intent.platform_address,
            amount: intent.amount,
            tax_amount: intent.tax_amount,
            gas_payer: intent.gas_payer,
            status: intent.status.as_str().to_string(),
            chain_id: intent.chain_id,
            contract_address: intent.contract_address,
            transaction_hash: intent.transaction_hash,
            log_index: intent.log_index,
            event_type: intent
                .event_type
                .map(|event_type| event_type.as_str().to_string()),
            external_transaction_id: intent.external_transaction_id,
            transaction_id: intent.transaction_id,
            wallet_provider: intent.wallet_provider,
            metamask_required: intent.metamask_required,
            wallet_action: intent.wallet_action,
            last_error: intent.last_error,
            created_at: intent.created_at,
            updated_at: intent.updated_at,
            credited_at: intent.credited_at,
        }
    }
}

impl From<WalletRewardRecordAudit> for WalletRewardRecordAuditResponse {
    fn from(record: WalletRewardRecordAudit) -> Self {
        Self {
            reward_candidate_id: record.reward_candidate_id,
            candidate_status: record.candidate_status.as_str().to_string(),
            reconciliation_status: record.reconciliation_status.as_str().to_string(),
            approved_amount: record.approved_amount,
            wallet_credit_record_id: record.wallet_credit_record_id,
            wallet_credit_transaction_id: record.wallet_credit_transaction_id,
            internal_transaction_id: record.internal_transaction_id,
            payout_record_id: record.payout_record_id,
            payout_transaction_id: record.payout_transaction_id,
            external_transaction_id: record.external_transaction_id,
            notification_id: record.notification_id,
            notified_at: record.notified_at,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

impl From<WalletCompensationRecordAudit> for WalletCompensationRecordAuditResponse {
    fn from(record: WalletCompensationRecordAudit) -> Self {
        Self {
            id: record.id,
            reward_candidate_id: record.reward_candidate_id,
            wallet_id: record.wallet_id,
            transaction_id: record.transaction_id,
            internal_transaction_id: record.internal_transaction_id,
            amount: record.amount,
            reason: record.reason,
            idempotency_key: record.idempotency_key,
            created_by_user_id: record.created_by_user_id,
            created_at: record.created_at,
        }
    }
}
