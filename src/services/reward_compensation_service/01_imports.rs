use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    internal_transactions, transactions, transactions_internal_transactions, wallets,
};
use crate::models::reward_compensation_record::{
    NewRewardCompensationRecord, RewardCompensationRecord,
};
use crate::models::transaction::{
    NewInternalTransaction, NewTransaction, NewTransactionInternalTransactionLink,
};
use crate::models::wallet::Wallet;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_candidate_repository;
use crate::repositories::reward_compensation_record_repository;
use crate::services::wallet_service;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

pub const REWARD_TRANSACTION_TYPE_COMPENSATION: &str = "reward_compensation";

#[derive(Debug, Clone)]
pub struct RewardCompensationRequest {
    pub reward_candidate_id: i64,
    pub amount: BigDecimal,
    pub reason: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone)]
pub struct RewardCompensationResult {
    pub record: RewardCompensationRecord,
    pub wallet: Wallet,
    pub created: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RewardCompensationError {
    PermissionDenied(String),
    InvalidInput(String),
    InsufficientFunds,
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for RewardCompensationError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardCompensationError::NotFound,
            other => RewardCompensationError::Database(other.to_string()),
        }
    }
}

pub async fn record_reward_compensation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: RewardCompensationRequest,
) -> Result<RewardCompensationResult, RewardCompensationError> {
    ensure_compensation_permission(conn, actor_user_id).await?;
    validate_compensation_request(&request)?;

    conn.transaction::<_, RewardCompensationError, _>(|conn| {
        Box::pin(async move {
            if let Some(existing) =
                reward_compensation_record_repository::find_reward_compensation_record_by_idempotency_key(
                    conn,
                    &request.idempotency_key,
                )
                .await?
            {
                let wallet = wallets::table
                    .find(existing.wallet_id)
                    .first::<Wallet>(conn)
                    .await?;
                return Ok(RewardCompensationResult {
                    record: existing,
                    wallet,
                    created: false,
                });
            }

            let candidate =
                reward_candidate_repository::find_candidate(conn, request.reward_candidate_id)
                    .await?;
            let linked_wallet = wallet_service::link_user_wallet(conn, candidate.student_user_id)
                .await
                .map_err(RewardCompensationError::from)?;
            let wallet = apply_wallet_adjustment(
                conn,
                linked_wallet.wallet.id,
                request.amount.clone(),
            )
            .await?;
            let internal_transaction_id =
                create_internal_transaction(conn, wallet.id, request.amount.clone()).await?;
            let transaction_id =
                create_compensation_transaction(conn, internal_transaction_id).await?;

            let record = reward_compensation_record_repository::create_reward_compensation_record(
                conn,
                NewRewardCompensationRecord {
                    reward_candidate_id: candidate.id,
                    wallet_id: wallet.id,
                    transaction_id,
                    internal_transaction_id,
                    amount: request.amount,
                    reason: request.reason.trim().to_string(),
                    idempotency_key: request.idempotency_key,
                    created_by_user_id: actor_user_id,
                },
            )
            .await?;

            Ok(RewardCompensationResult {
                record,
                wallet,
                created: true,
            })
        })
    })
    .await
}

async fn ensure_compensation_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<(), RewardCompensationError> {
    let permissions = [
        Permissions::RECONCILE_WALLETS.to_string(),
        Permissions::MANAGE_WALLETS.to_string(),
    ];

    for permission in permissions {
        if user_permission_platform_request(conn, actor_user_id, &permission).await? {
            return Ok(());
        }
    }

    Err(RewardCompensationError::PermissionDenied(
        Permissions::RECONCILE_WALLETS.to_string(),
    ))
}
