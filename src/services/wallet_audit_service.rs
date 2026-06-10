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

async fn load_internal_transactions(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> QueryResult<Vec<WalletInternalTransactionAudit>> {
    let rows = internal_transactions::table
        .inner_join(
            transactions_internal_transactions::table.on(internal_transactions::id
                .eq(transactions_internal_transactions::internal_transaction_id)),
        )
        .inner_join(
            transactions::table
                .on(transactions_internal_transactions::transaction_id.eq(transactions::id)),
        )
        .filter(internal_transactions::wallet_id.eq(wallet_id))
        .select((
            internal_transactions::id,
            transactions::id,
            transactions::type_,
            internal_transactions::amount,
            transactions::created_at,
        ))
        .order(transactions::created_at.desc())
        .load::<(i64, i64, String, bigdecimal::BigDecimal, DateTime<Utc>)>(conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(
            |(internal_transaction_id, transaction_id, transaction_type, amount, created_at)| {
                WalletInternalTransactionAudit {
                    internal_transaction_id,
                    transaction_id,
                    transaction_type,
                    amount: amount.to_string(),
                    created_at,
                }
            },
        )
        .collect())
}

async fn load_wallet_reward_candidate_ids(
    conn: &mut AsyncPgConnection,
    wallet: &Wallet,
) -> QueryResult<Vec<i64>> {
    let mut candidate_ids = HashSet::new();

    let credit_candidate_ids = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::wallet_id.eq(wallet.id))
        .select(reward_wallet_credit_records::reward_candidate_id)
        .load::<i64>(conn)
        .await?;
    candidate_ids.extend(credit_candidate_ids);

    if let Some(user_id) = wallet.user_id {
        let user_candidate_ids = reward_candidates::table
            .filter(reward_candidates::student_user_id.eq(user_id))
            .select(reward_candidates::id)
            .load::<i64>(conn)
            .await?;
        candidate_ids.extend(user_candidate_ids);
    }

    if let Some(organization_id) = wallet.organization_id {
        let organization_candidate_ids = reward_candidates::table
            .filter(reward_candidates::source_organization_id.eq(organization_id))
            .select(reward_candidates::id)
            .load::<i64>(conn)
            .await?;
        candidate_ids.extend(organization_candidate_ids);
    }

    let mut candidate_ids = candidate_ids.into_iter().collect::<Vec<_>>();
    candidate_ids.sort_unstable();
    Ok(candidate_ids)
}

async fn load_reward_records(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
) -> QueryResult<Vec<WalletRewardRecordAudit>> {
    if candidate_ids.is_empty() {
        return Ok(Vec::new());
    }

    let candidates = reward_candidates::table
        .filter(reward_candidates::id.eq_any(candidate_ids))
        .order(reward_candidates::created_at.desc())
        .load::<RewardCandidate>(conn)
        .await?;

    let credit_records = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq_any(candidate_ids))
        .load::<RewardWalletCreditRecord>(conn)
        .await?
        .into_iter()
        .map(|record| (record.reward_candidate_id, record))
        .collect::<HashMap<_, _>>();

    let payout_records = reward_payout_records::table
        .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
        .load::<RewardPayoutRecord>(conn)
        .await?
        .into_iter()
        .map(|record| (record.reward_candidate_id, record))
        .collect::<HashMap<_, _>>();

    Ok(candidates
        .into_iter()
        .map(|candidate| {
            let credit_record = credit_records.get(&candidate.id);
            let payout_record = payout_records.get(&candidate.id);
            WalletRewardRecordAudit {
                reward_candidate_id: candidate.id,
                candidate_status: candidate.status.clone(),
                reconciliation_status: reward_reconciliation_status(
                    &candidate,
                    credit_record,
                    payout_record,
                ),
                approved_amount: candidate.approved_amount.as_ref().map(ToString::to_string),
                wallet_credit_record_id: credit_record.map(|record| record.id),
                wallet_credit_transaction_id: credit_record.map(|record| record.transaction_id),
                internal_transaction_id: credit_record.map(|record| record.internal_transaction_id),
                payout_record_id: payout_record.map(|record| record.id),
                payout_transaction_id: payout_record.map(|record| record.transaction_id),
                external_transaction_id: payout_record.map(|record| record.external_transaction_id),
                notification_id: credit_record.and_then(|record| record.notification_id),
                notified_at: credit_record.and_then(|record| record.notified_at),
                created_at: candidate.created_at,
                updated_at: candidate.updated_at,
            }
        })
        .collect())
}

async fn load_external_transactions(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
    wallet_transaction_ids: &[i64],
) -> QueryResult<Vec<WalletExternalTransactionAudit>> {
    let mut audits = Vec::new();
    let mut seen = HashSet::new();

    if !candidate_ids.is_empty() {
        let rows =
            reward_payout_records::table
                .inner_join(external_transactions::table.on(
                    reward_payout_records::external_transaction_id.eq(external_transactions::id),
                ))
                .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
                .select((
                    reward_payout_records::reward_candidate_id,
                    reward_payout_records::transaction_id,
                    external_transactions::id,
                    external_transactions::amount,
                    external_transactions::blockchain_address,
                    external_transactions::chain_id,
                    external_transactions::contract_address,
                    external_transactions::transaction_hash,
                    external_transactions::log_index,
                    external_transactions::event_type,
                    external_transactions::from_address,
                    external_transactions::to_address,
                ))
                .order(external_transactions::id.desc())
                .load::<(
                    i64,
                    i64,
                    i64,
                    bigdecimal::BigDecimal,
                    String,
                    Option<i64>,
                    Option<String>,
                    Option<String>,
                    Option<i64>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                )>(conn)
                .await?;

        for (
            reward_candidate_id,
            transaction_id,
            external_transaction_id,
            amount,
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
        ) in rows
        {
            seen.insert((transaction_id, external_transaction_id));
            audits.push(WalletExternalTransactionAudit {
                external_transaction_id,
                transaction_id,
                reward_candidate_id: Some(reward_candidate_id),
                amount: amount.to_string(),
                blockchain_address,
                chain_id,
                contract_address,
                transaction_hash,
                log_index,
                event_type,
                from_address,
                to_address,
            });
        }
    }

    if !wallet_transaction_ids.is_empty() {
        let rows = transactions_external_transactions::table
            .inner_join(
                external_transactions::table
                    .on(transactions_external_transactions::external_transaction_id
                        .eq(external_transactions::id)),
            )
            .filter(
                transactions_external_transactions::transaction_id.eq_any(wallet_transaction_ids),
            )
            .select((
                transactions_external_transactions::transaction_id,
                external_transactions::id,
                external_transactions::amount,
                external_transactions::blockchain_address,
                external_transactions::chain_id,
                external_transactions::contract_address,
                external_transactions::transaction_hash,
                external_transactions::log_index,
                external_transactions::event_type,
                external_transactions::from_address,
                external_transactions::to_address,
            ))
            .order(external_transactions::id.desc())
            .load::<(
                i64,
                i64,
                bigdecimal::BigDecimal,
                String,
                Option<i64>,
                Option<String>,
                Option<String>,
                Option<i64>,
                Option<String>,
                Option<String>,
                Option<String>,
            )>(conn)
            .await?;

        for (
            transaction_id,
            external_transaction_id,
            amount,
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
        ) in rows
        {
            if seen.insert((transaction_id, external_transaction_id)) {
                audits.push(WalletExternalTransactionAudit {
                    external_transaction_id,
                    transaction_id,
                    reward_candidate_id: None,
                    amount: amount.to_string(),
                    blockchain_address,
                    chain_id,
                    contract_address,
                    transaction_hash,
                    log_index,
                    event_type,
                    from_address,
                    to_address,
                });
            }
        }
    }

    audits.sort_by(|left, right| {
        right
            .external_transaction_id
            .cmp(&left.external_transaction_id)
            .then_with(|| right.transaction_id.cmp(&left.transaction_id))
    });

    Ok(audits)
}

async fn load_compensation_records(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> QueryResult<Vec<WalletCompensationRecordAudit>> {
    let records = reward_compensation_records::table
        .filter(reward_compensation_records::wallet_id.eq(wallet_id))
        .order(reward_compensation_records::created_at.desc())
        .load::<RewardCompensationRecord>(conn)
        .await?;

    Ok(records
        .into_iter()
        .map(|record| WalletCompensationRecordAudit {
            id: record.id,
            reward_candidate_id: record.reward_candidate_id,
            wallet_id: record.wallet_id,
            transaction_id: record.transaction_id,
            internal_transaction_id: record.internal_transaction_id,
            amount: record.amount.to_string(),
            reason: record.reason,
            idempotency_key: record.idempotency_key,
            created_by_user_id: record.created_by_user_id,
            created_at: record.created_at,
        })
        .collect())
}

fn reward_reconciliation_status(
    candidate: &RewardCandidate,
    credit_record: Option<&RewardWalletCreditRecord>,
    payout_record: Option<&RewardPayoutRecord>,
) -> String {
    let status = match candidate.status.as_str() {
        REWARD_STATUS_NEEDS_RECONCILIATION => "needs_reconciliation",
        REWARD_STATUS_TOKEN_CONFIRMED if payout_record.is_none() => "needs_payout_record",
        REWARD_STATUS_TOKEN_CONFIRMED if credit_record.is_none() => "needs_wallet_credit",
        REWARD_STATUS_WALLET_CREDITED if credit_record.is_none() => "needs_wallet_credit_record",
        REWARD_STATUS_WALLET_CREDITED => "needs_notification",
        REWARD_STATUS_NOTIFIED | REWARD_STATUS_COMPLETED
            if credit_record
                .and_then(|record| record.notification_id)
                .is_none() =>
        {
            "needs_notification_record"
        }
        REWARD_STATUS_NOTIFIED | REWARD_STATUS_COMPLETED => "reconciled",
        REWARD_STATUS_AMOUNT_APPROVED | REWARD_STATUS_TOKEN_PENDING => "pending_execution",
        REWARD_STATUS_AMOUNT_REJECTED | REWARD_STATUS_TEACHER_REJECTED | REWARD_STATUS_FAILED => {
            "closed_without_payout"
        }
        REWARD_STATUS_PENDING_TEACHER_APPROVAL | REWARD_STATUS_TEACHER_APPROVED => {
            "pending_decision"
        }
        _ if credit_record.is_some() => "reconciled",
        _ => "pending",
    };

    status.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::reward_candidate::{
        REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED,
        REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION,
        REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        REWARD_STATUS_TEACHER_APPROVED, REWARD_STATUS_TEACHER_REJECTED,
        REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING, REWARD_STATUS_WALLET_CREDITED,
    };
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use serde_json::json;

    fn candidate(status: &str) -> RewardCandidate {
        RewardCandidate {
            id: 1, course_id: 1, student_user_id: 2, submitter_user_id: 3,
            source_scope: "course".into(), source_organization_id: None,
            event_type: "course_completion".into(),
            idempotency_key: "key".into(),
            evidence: json!({"completion_percentage": 100.0}),
            status: status.into(),
            teacher_approver_user_id: None, teacher_decision_reason: None,
            teacher_decided_at: None,
            amount_reviewer_user_id: None,
            approved_amount: Some(BigDecimal::from(100)),
            amount_decision_reason: None, amount_decided_at: None,
            created_at: Utc::now(), updated_at: Utc::now(),
        }
    }

    fn credit_record(notification_id: Option<i64>) -> RewardWalletCreditRecord {
        RewardWalletCreditRecord {
            id: 1, reward_candidate_id: 1, wallet_id: 1,
            transaction_id: 1, internal_transaction_id: 1,
            notification_id, notified_at: None,
            created_at: Utc::now(),
        }
    }

    fn payout_record() -> RewardPayoutRecord {
        RewardPayoutRecord {
            id: 1, reward_candidate_id: 1,
            transaction_id: 1, external_transaction_id: 1,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn needs_reconciliation_status() {
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_NEEDS_RECONCILIATION), None, None),
            "needs_reconciliation"
        );
    }

    #[test]
    fn token_confirmed_without_payout_record() {
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_TOKEN_CONFIRMED), None, None),
            "needs_payout_record"
        );
    }

    #[test]
    fn token_confirmed_without_credit_record() {
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_TOKEN_CONFIRMED), None, Some(&payout_record())),
            "needs_wallet_credit"
        );
    }

    #[test]
    fn wallet_credited_without_credit_record() {
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_WALLET_CREDITED), None, None),
            "needs_wallet_credit_record"
        );
    }

    #[test]
    fn wallet_credited_needs_notification() {
        assert_eq!(
            reward_reconciliation_status(
                &candidate(REWARD_STATUS_WALLET_CREDITED),
                Some(&credit_record(None)),
                None,
            ),
            "needs_notification"
        );
    }

    #[test]
    fn notified_without_notification_record() {
        let cr = credit_record(None);
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_NOTIFIED), Some(&cr), None),
            "needs_notification_record"
        );
    }

    #[test]
    fn notified_with_notification_is_reconciled() {
        assert_eq!(
            reward_reconciliation_status(
                &candidate(REWARD_STATUS_NOTIFIED),
                Some(&credit_record(Some(10))),
                None,
            ),
            "reconciled"
        );
    }

    #[test]
    fn completed_with_all_records_is_reconciled() {
        assert_eq!(
            reward_reconciliation_status(
                &candidate(REWARD_STATUS_COMPLETED),
                Some(&credit_record(Some(10))),
                None,
            ),
            "reconciled"
        );
    }

    #[test]
    fn amount_approved_is_pending_execution() {
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_AMOUNT_APPROVED), None, None),
            "pending_execution"
        );
    }

    #[test]
    fn token_pending_is_pending_execution() {
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_TOKEN_PENDING), None, None),
            "pending_execution"
        );
    }

    #[test]
    fn rejected_is_closed_without_payout() {
        for status in &[
            REWARD_STATUS_AMOUNT_REJECTED,
            REWARD_STATUS_TEACHER_REJECTED,
            REWARD_STATUS_FAILED,
        ] {
            assert_eq!(
                reward_reconciliation_status(&candidate(status), None, None),
                "closed_without_payout"
            );
        }
    }

    #[test]
    fn pending_approval_is_pending_decision() {
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_PENDING_TEACHER_APPROVAL), None, None),
            "pending_decision"
        );
        assert_eq!(
            reward_reconciliation_status(&candidate(REWARD_STATUS_TEACHER_APPROVED), None, None),
            "pending_decision"
        );
    }

    #[test]
    fn any_status_with_credit_record_is_reconciled() {
        let cr = credit_record(Some(5));
        assert_eq!(
            reward_reconciliation_status(&candidate("custom_status"), Some(&cr), None),
            "reconciled"
        );
    }

    #[test]
    fn unknown_status_without_record_is_pending() {
        assert_eq!(
            reward_reconciliation_status(&candidate("custom_status"), None, None),
            "pending"
        );
    }
}
