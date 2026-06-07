use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, external_transactions, internal_transactions, reward_candidates,
    reward_payout_records, reward_wallet_credit_records,
};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_ADJUSTED, REWARD_STATUS_AMOUNT_APPROVED,
    REWARD_STATUS_AMOUNT_REJECTED, REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED,
    REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::repositories::course_repository::user_permission_course_request;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

const DEFAULT_STUDENT_REWARD_HISTORY_LIMIT: i64 = 50;
const MAX_STUDENT_REWARD_HISTORY_LIMIT: i64 = 100;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct StudentRewardHistoryRequest {
    pub course_id: Option<i32>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl StudentRewardHistoryRequest {
    fn limit(&self) -> i64 {
        self.limit
            .unwrap_or(DEFAULT_STUDENT_REWARD_HISTORY_LIMIT)
            .clamp(1, MAX_STUDENT_REWARD_HISTORY_LIMIT)
    }

    fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

#[derive(Debug, Serialize)]
pub struct StudentRewardHistoryEntry {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub course_title: String,
    pub event_type: String,
    pub status: String,
    pub approved_amount: Option<String>,
    pub wallet_credit: Option<StudentRewardWalletCredit>,
    pub token_transaction: Option<StudentRewardTokenTransaction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StudentRewardWalletCredit {
    pub reward_wallet_credit_record_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: String,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub credited_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StudentRewardTokenTransaction {
    pub reward_payout_record_id: i64,
    pub payout_transaction_id: i64,
    pub external_transaction_id: i64,
    pub amount: String,
    pub blockchain_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StudentRewardHistoryError {
    InvalidInput(String),
    Database(String),
}

impl From<diesel::result::Error> for StudentRewardHistoryError {
    fn from(error: diesel::result::Error) -> Self {
        StudentRewardHistoryError::Database(error.to_string())
    }
}

pub async fn list_student_reward_history(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: StudentRewardHistoryRequest,
) -> Result<Vec<StudentRewardHistoryEntry>, StudentRewardHistoryError> {
    let status = match request.status.as_deref() {
        Some(status) => Some(normalize_reward_status(status)?),
        None => None,
    };

    let mut query = reward_candidates::table
        .inner_join(courses::table.on(reward_candidates::course_id.eq(courses::id)))
        .filter(reward_candidates::student_user_id.eq(actor_user_id))
        .into_boxed();

    if let Some(course_id) = request.course_id {
        query = query.filter(reward_candidates::course_id.eq(course_id));
    }

    if let Some(status) = status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    let rows = query
        .select((RewardCandidate::as_select(), courses::title))
        .order(reward_candidates::created_at.desc())
        .limit(request.limit())
        .offset(request.offset())
        .load::<(RewardCandidate, String)>(conn)
        .await?;

    let mut history = Vec::new();
    for (candidate, course_title) in rows {
        let can_view_reward_status = user_permission_course_request(
            conn,
            actor_user_id,
            candidate.course_id,
            &Permissions::VIEW_COURSE_REWARD_STATUS.to_string(),
        )
        .await?;

        if !can_view_reward_status {
            continue;
        }

        let wallet_credit = load_wallet_credit(conn, candidate.id).await?;
        let token_transaction = load_token_transaction(conn, candidate.id).await?;
        history.push(StudentRewardHistoryEntry {
            reward_candidate_id: candidate.id,
            course_id: candidate.course_id,
            course_title,
            event_type: candidate.event_type,
            status: candidate.status,
            approved_amount: candidate
                .approved_amount
                .as_ref()
                .map(std::string::ToString::to_string),
            wallet_credit,
            token_transaction,
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        });
    }

    Ok(history)
}

async fn load_wallet_credit(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> Result<Option<StudentRewardWalletCredit>, StudentRewardHistoryError> {
    type WalletCreditRow = (
        i64,
        i32,
        i64,
        i64,
        Option<i64>,
        Option<DateTime<Utc>>,
        DateTime<Utc>,
        BigDecimal,
    );

    let row = reward_wallet_credit_records::table
        .inner_join(internal_transactions::table.on(
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transactions::id),
        ))
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(reward_candidate_id))
        .select((
            reward_wallet_credit_records::id,
            reward_wallet_credit_records::wallet_id,
            reward_wallet_credit_records::transaction_id,
            reward_wallet_credit_records::internal_transaction_id,
            reward_wallet_credit_records::notification_id,
            reward_wallet_credit_records::notified_at,
            reward_wallet_credit_records::created_at,
            internal_transactions::amount,
        ))
        .first::<WalletCreditRow>(conn)
        .await
        .optional()?;

    Ok(row.map(
        |(
            reward_wallet_credit_record_id,
            wallet_id,
            transaction_id,
            internal_transaction_id,
            notification_id,
            notified_at,
            credited_at,
            amount,
        )| StudentRewardWalletCredit {
            reward_wallet_credit_record_id,
            wallet_id,
            transaction_id,
            internal_transaction_id,
            amount: amount.to_string(),
            notification_id,
            notified_at,
            credited_at,
        },
    ))
}

async fn load_token_transaction(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> Result<Option<StudentRewardTokenTransaction>, StudentRewardHistoryError> {
    type TokenTransactionRow = (
        i64,
        i64,
        i64,
        DateTime<Utc>,
        BigDecimal,
        String,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<String>,
    );

    let row = reward_payout_records::table
        .inner_join(
            external_transactions::table
                .on(reward_payout_records::external_transaction_id.eq(external_transactions::id)),
        )
        .filter(reward_payout_records::reward_candidate_id.eq(reward_candidate_id))
        .select((
            reward_payout_records::id,
            reward_payout_records::transaction_id,
            reward_payout_records::external_transaction_id,
            reward_payout_records::created_at,
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
        .first::<TokenTransactionRow>(conn)
        .await
        .optional()?;

    Ok(row.map(
        |(
            reward_payout_record_id,
            payout_transaction_id,
            external_transaction_id,
            recorded_at,
            amount,
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
        )| StudentRewardTokenTransaction {
            reward_payout_record_id,
            payout_transaction_id,
            external_transaction_id,
            amount: amount.to_string(),
            blockchain_address,
            chain_id,
            contract_address,
            transaction_hash,
            log_index,
            event_type,
            from_address,
            to_address,
            recorded_at,
        },
    ))
}

fn normalize_reward_status(status: &str) -> Result<String, StudentRewardHistoryError> {
    let normalized = status.trim().to_ascii_lowercase().replace(['-', ' '], "_");
    match normalized.as_str() {
        REWARD_STATUS_PENDING_TEACHER_APPROVAL
        | REWARD_STATUS_TEACHER_APPROVED
        | REWARD_STATUS_TEACHER_REJECTED
        | REWARD_STATUS_AMOUNT_APPROVED
        | REWARD_STATUS_AMOUNT_REJECTED
        | REWARD_STATUS_ADJUSTED
        | REWARD_STATUS_TOKEN_PENDING
        | REWARD_STATUS_TOKEN_CONFIRMED
        | REWARD_STATUS_WALLET_CREDITED
        | REWARD_STATUS_NOTIFIED
        | REWARD_STATUS_COMPLETED
        | REWARD_STATUS_NEEDS_RECONCILIATION
        | REWARD_STATUS_FAILED => Ok(normalized),
        _ => Err(StudentRewardHistoryError::InvalidInput(
            "unsupported reward candidate status".to_string(),
        )),
    }
}
