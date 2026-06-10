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

fn validate_compensation_request(
    request: &RewardCompensationRequest,
) -> Result<(), RewardCompensationError> {
    if request.amount == BigDecimal::from(0) {
        return Err(RewardCompensationError::InvalidInput(
            "compensation amount cannot be zero".to_string(),
        ));
    }
    if request.reason.trim().is_empty() {
        return Err(RewardCompensationError::InvalidInput(
            "compensation reason is required".to_string(),
        ));
    }
    if request.idempotency_key.trim().is_empty() {
        return Err(RewardCompensationError::InvalidInput(
            "compensation idempotency key is required".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    fn compensation_request(amount: i64, reason: &str, idempotency_key: &str) -> RewardCompensationRequest {
        RewardCompensationRequest {
            reward_candidate_id: 1,
            amount: BigDecimal::from(amount),
            reason: reason.into(),
            idempotency_key: idempotency_key.into(),
        }
    }

    #[test]
    fn valid_request_passes() {
        validate_compensation_request(
            &compensation_request(100, "Adjustment", "key-1"),
        ).unwrap();
    }

    #[test]
    fn negative_amount_is_valid() {
        validate_compensation_request(
            &compensation_request(-50, "Negative comp", "key-2"),
        ).unwrap();
    }

    #[test]
    fn zero_amount_fails() {
        assert!(
            validate_compensation_request(&compensation_request(0, "reason", "key")).is_err()
        );
    }

    #[test]
    fn empty_reason_fails() {
        assert!(
            validate_compensation_request(
                &compensation_request(100, "   ", "key")
            ).is_err()
        );
    }

    #[test]
    fn empty_idempotency_key_fails() {
        assert!(
            validate_compensation_request(
                &compensation_request(100, "reason", "")
            ).is_err()
        );
    }
}

async fn apply_wallet_adjustment(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> Result<Wallet, RewardCompensationError> {
    let updated_wallet = diesel::update(
        wallets::table.filter(
            wallets::id
                .eq(wallet_id)
                .and((wallets::value + amount.clone()).ge(BigDecimal::from(0))),
        ),
    )
    .set(wallets::value.eq(wallets::value + amount))
    .get_result::<Wallet>(conn)
    .await;

    match updated_wallet {
        Ok(wallet) => Ok(wallet),
        Err(diesel::result::Error::NotFound) => Err(RewardCompensationError::InsufficientFunds),
        Err(error) => Err(RewardCompensationError::from(error)),
    }
}

async fn create_internal_transaction(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> Result<i64, RewardCompensationError> {
    diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .map_err(RewardCompensationError::from)
}

async fn create_compensation_transaction(
    conn: &mut AsyncPgConnection,
    internal_transaction_id: i64,
) -> Result<i64, RewardCompensationError> {
    let transaction_id = diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: REWARD_TRANSACTION_TYPE_COMPENSATION,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(transaction_id)
}
