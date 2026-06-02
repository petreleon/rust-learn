use crate::db::schema::{
    courses_organizations, internal_transactions, reward_candidates, reward_policies, transactions,
    transactions_internal_transactions, wallets,
};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_TOKEN_CONFIRMED,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_policy::{
    RewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::models::transaction::{
    NewInternalTransaction, NewTransaction, NewTransactionInternalTransactionLink,
};
use crate::models::wallet::Wallet;
use crate::repositories::persistent_state_repository::get_persistent_state;
use crate::repositories::reward_candidate_repository;
use crate::services::wallet_service;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::AsyncConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub const REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER: &str = "presigner_transfer";
pub const REWARD_PAYOUT_METHOD_TREASURY_TRANSFER: &str = "treasury_transfer";
pub const REWARD_PAYOUT_METHOD_MINT: &str = "mint";
pub const REWARD_PAYOUT_METHOD_OFF_CHAIN: &str = "off_chain";
pub const REWARD_TRANSACTION_TYPE_WALLET_CREDIT: &str = "reward_wallet_credit";

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutPlan {
    pub candidate_id: i64,
    pub policy_id: i64,
    pub amount: BigDecimal,
    pub payment_strategy: String,
    pub payout_method: String,
    pub requires_token_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardWalletCreditResult {
    pub candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub amount: BigDecimal,
    pub credited: bool,
}

#[derive(Debug, PartialEq)]
pub enum RewardExecutionError {
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    Database(String),
}

impl From<diesel::result::Error> for RewardExecutionError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardExecutionError::NoActivePolicy,
            other => RewardExecutionError::Database(other.to_string()),
        }
    }
}

pub async fn plan_reward_payout(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    let candidate = reward_candidate_repository::find_candidate(conn, candidate_id).await?;
    ensure_candidate_ready_for_payout(&candidate)?;

    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardExecutionError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardExecutionError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }

    let policy = resolve_active_reward_policy(conn, candidate.course_id, &candidate.event_type)
        .await?
        .ok_or(RewardExecutionError::NoActivePolicy)?;
    let payout_method = select_payout_method(conn, &policy).await?;
    let requires_token_confirmation = payout_method != REWARD_PAYOUT_METHOD_OFF_CHAIN;

    Ok(RewardPayoutPlan {
        candidate_id: candidate.id,
        policy_id: policy.id,
        amount,
        payment_strategy: policy.payment_strategy,
        payout_method,
        requires_token_confirmation,
    })
}

pub async fn credit_reward_wallet(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardWalletCreditResult, RewardExecutionError> {
    conn.transaction::<_, RewardExecutionError, _>(|conn| {
        Box::pin(async move {
            let candidate = reward_candidate_repository::find_candidate(conn, candidate_id).await?;
            let amount = approved_positive_amount(&candidate)?;

            if candidate.status == REWARD_STATUS_WALLET_CREDITED {
                let wallet = wallet_service::find_user_wallet(conn, candidate.student_user_id)
                    .await?
                    .ok_or_else(|| {
                        RewardExecutionError::InvalidStatus(
                            "wallet credited candidate is missing a user wallet".to_string(),
                        )
                    })?;
                return Ok(RewardWalletCreditResult {
                    candidate_id: candidate.id,
                    wallet_id: wallet.id,
                    transaction_id: None,
                    internal_transaction_id: None,
                    amount,
                    credited: false,
                });
            }

            ensure_wallet_credit_allowed(conn, &candidate).await?;

            let wallet = wallet_service::link_user_wallet(conn, candidate.student_user_id)
                .await?
                .wallet;
            let wallet = credit_wallet_balance(conn, wallet.id, amount.clone()).await?;
            let internal_transaction_id =
                create_internal_transaction(conn, wallet.id, amount.clone()).await?;
            let transaction_id =
                create_wallet_credit_transaction(conn, internal_transaction_id).await?;
            mark_candidate_wallet_credited(conn, candidate.id).await?;

            Ok(RewardWalletCreditResult {
                candidate_id: candidate.id,
                wallet_id: wallet.id,
                transaction_id: Some(transaction_id),
                internal_transaction_id: Some(internal_transaction_id),
                amount,
                credited: true,
            })
        })
    })
    .await
}

fn ensure_candidate_ready_for_payout(
    candidate: &RewardCandidate,
) -> Result<(), RewardExecutionError> {
    if candidate.status == REWARD_STATUS_AMOUNT_APPROVED {
        Ok(())
    } else {
        Err(RewardExecutionError::InvalidStatus(
            "reward candidate must be amount approved before payout planning".to_string(),
        ))
    }
}

fn approved_positive_amount(
    candidate: &RewardCandidate,
) -> Result<BigDecimal, RewardExecutionError> {
    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardExecutionError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardExecutionError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }
    Ok(amount)
}

async fn ensure_wallet_credit_allowed(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
) -> Result<(), RewardExecutionError> {
    if candidate.status == REWARD_STATUS_TOKEN_CONFIRMED {
        return Ok(());
    }

    if candidate.status == REWARD_STATUS_AMOUNT_APPROVED {
        let plan = plan_reward_payout(conn, candidate.id).await?;
        if plan.payout_method == REWARD_PAYOUT_METHOD_OFF_CHAIN {
            return Ok(());
        }
    }

    Err(RewardExecutionError::InvalidStatus(
        "wallet credit requires token confirmation unless the reward policy is off-chain"
            .to_string(),
    ))
}

async fn credit_wallet_balance(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> QueryResult<Wallet> {
    diesel::update(wallets::table.find(wallet_id))
        .set(wallets::value.eq(wallets::value + amount))
        .get_result(conn)
        .await
}

async fn create_internal_transaction(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> QueryResult<i64> {
    diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
}

async fn create_wallet_credit_transaction(
    conn: &mut AsyncPgConnection,
    internal_transaction_id: i64,
) -> QueryResult<i64> {
    let transaction_id = diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: REWARD_TRANSACTION_TYPE_WALLET_CREDIT,
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

async fn mark_candidate_wallet_credited(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(REWARD_STATUS_WALLET_CREDITED),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}

async fn select_payout_method(
    conn: &mut AsyncPgConnection,
    policy: &RewardPolicy,
) -> Result<String, RewardExecutionError> {
    match policy.payment_strategy.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER => {
            if has_presigner_contract(conn).await? {
                Ok(REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER.to_string())
            } else {
                Ok(REWARD_PAYOUT_METHOD_TREASURY_TRANSFER.to_string())
            }
        }
        REWARD_PAYMENT_MINT => Ok(REWARD_PAYOUT_METHOD_MINT.to_string()),
        REWARD_PAYMENT_OFF_CHAIN => Ok(REWARD_PAYOUT_METHOD_OFF_CHAIN.to_string()),
        _ => Err(RewardExecutionError::InvalidInput(
            "unsupported reward payment strategy".to_string(),
        )),
    }
}

async fn has_presigner_contract(
    conn: &mut AsyncPgConnection,
) -> Result<bool, RewardExecutionError> {
    let address = get_persistent_state(conn, "learn_token_presigner_address").await?;
    Ok(address
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false))
}

async fn resolve_active_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> QueryResult<Option<RewardPolicy>> {
    if let Some(policy) = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()?
    {
        return Ok(Some(policy));
    }

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        if let Some(policy) = reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
            .filter(reward_policies::organization_id.eq_any(organization_ids))
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true))
            .order((
                reward_policies::version.desc(),
                reward_policies::created_at.desc(),
            ))
            .first::<RewardPolicy>(conn)
            .await
            .optional()?
        {
            return Ok(Some(policy));
        }
    }

    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
        .filter(reward_policies::organization_id.is_null())
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()
}
