use crate::db::schema::{
    external_transactions, internal_transactions, reward_candidates, reward_compensation_records,
    reward_payout_records, reward_wallet_credit_records, transactions,
    transactions_external_transactions, transactions_internal_transactions,
};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED,
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION,
    REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_compensation_record::RewardCompensationRecord;
use crate::models::reward_payout_record::RewardPayoutRecord;
use crate::models::reward_wallet_credit_record::RewardWalletCreditRecord;
use crate::models::wallet::Wallet;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize)]
pub struct WalletAudit {
    pub wallet: WalletAuditWallet,
    pub internal_transactions: Vec<WalletInternalTransactionAudit>,
    pub external_transactions: Vec<WalletExternalTransactionAudit>,
    pub reward_records: Vec<WalletRewardRecordAudit>,
    pub compensation_records: Vec<WalletCompensationRecordAudit>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WalletAuditWallet {
    pub id: i32,
    pub owner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WalletInternalTransactionAudit {
    pub internal_transaction_id: i64,
    pub transaction_id: i64,
    pub transaction_type: String,
    pub amount: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WalletExternalTransactionAudit {
    pub external_transaction_id: i64,
    pub transaction_id: i64,
    pub reward_candidate_id: Option<i64>,
    pub amount: String,
    pub blockchain_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WalletRewardRecordAudit {
    pub reward_candidate_id: i64,
    pub candidate_status: String,
    pub reconciliation_status: String,
    pub approved_amount: Option<String>,
    pub wallet_credit_record_id: Option<i64>,
    pub wallet_credit_transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub payout_record_id: Option<i64>,
    pub payout_transaction_id: Option<i64>,
    pub external_transaction_id: Option<i64>,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WalletCompensationRecordAudit {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: String,
    pub reason: String,
    pub idempotency_key: String,
    pub created_by_user_id: i32,
    pub created_at: DateTime<Utc>,
}

pub async fn build_wallet_audit(
    conn: &mut AsyncPgConnection,
    wallet: Wallet,
) -> QueryResult<WalletAudit> {
    let internal_transactions = load_internal_transactions(conn, wallet.id).await?;
    let wallet_transaction_ids = internal_transactions
        .iter()
        .map(|row| row.transaction_id)
        .collect::<Vec<_>>();
    let candidate_ids = load_wallet_reward_candidate_ids(conn, &wallet).await?;
    let reward_records = load_reward_records(conn, candidate_ids.as_slice()).await?;
    let external_transactions = load_external_transactions(
        conn,
        candidate_ids.as_slice(),
        wallet_transaction_ids.as_slice(),
    )
    .await?;
    let compensation_records = load_compensation_records(conn, wallet.id).await?;

    Ok(WalletAudit {
        wallet: WalletAuditWallet::from(&wallet),
        internal_transactions,
        external_transactions,
        reward_records,
        compensation_records,
    })
}

impl From<&Wallet> for WalletAuditWallet {
    fn from(wallet: &Wallet) -> Self {
        let owner_type = if wallet.user_id.is_some() {
            "user"
        } else {
            "organization"
        };

        WalletAuditWallet {
            id: wallet.id,
            owner_type: owner_type.to_string(),
            user_id: wallet.user_id,
            organization_id: wallet.organization_id,
            value: wallet.value.to_string(),
        }
    }
}
