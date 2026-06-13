use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::credit_wallet::{
    RewardWalletCredit, RewardWalletCreditError, RewardWalletCreditOutput,
};
use crate::db::schema::reward_candidates;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::rewards::reward_wallet_credit_mappers::{
    map_diesel_error, RewardWalletCreditTransactionError,
};
use crate::infra::postgres::rewards::reward_wallet_credit_transactions::{
    create_internal_transaction, create_wallet_credit_transaction,
};
use crate::infra::postgres::rewards::reward_wallet_credit_validation::{
    approved_positive_amount, ensure_wallet_credit_allowed, reject_already_credited_without_record,
};
use crate::infra::postgres::rewards::reward_wallet_credit_wallets::{
    credit_wallet_balance, link_user_wallet,
};
use crate::models::reward_audit_event::{NewRewardAuditEvent, REWARD_AUDIT_EVENT_WALLET_CREDITED};
use crate::models::reward_candidate::RewardCandidate;
use crate::models::reward_wallet_credit_record::NewRewardWalletCreditRecord;
use crate::repositories::{
    reward_audit_event_repository, reward_candidate_repository,
    reward_wallet_credit_record_repository,
};

pub(super) async fn credit_reward_wallet(
    conn: &mut AsyncPgConnection,
    credit: RewardWalletCredit,
) -> Result<RewardWalletCreditOutput, RewardWalletCreditTransactionError> {
    let candidate = reward_candidate_repository::find_candidate(conn, credit.candidate_id).await?;
    credit_reward_wallet_for_candidate(
        conn,
        &candidate,
        credit.allow_reconciliation_credit,
        credit.actor_user_id,
    )
    .await
    .map_err(RewardWalletCreditTransactionError::from)
}

pub(crate) async fn credit_reward_wallet_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_credit: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditOutput, RewardWalletCreditError> {
    let amount = approved_positive_amount(candidate)?;
    let existing_record =
        reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
            conn,
            candidate.id,
        )
        .await
        .map_err(map_diesel_error)?;

    if let Some(record) = existing_record.as_ref() {
        return Ok(RewardWalletCreditOutput {
            candidate_id: candidate.id,
            wallet_id: record.wallet_id,
            credit_record_id: Some(record.id),
            transaction_id: Some(record.transaction_id),
            internal_transaction_id: Some(record.internal_transaction_id),
            amount,
            credited: false,
        });
    }

    reject_already_credited_without_record(candidate)?;
    ensure_wallet_credit_allowed(conn, candidate, allow_reconciliation_credit).await?;

    let wallet = link_user_wallet(conn, candidate.student_user_id).await?;
    let wallet = credit_wallet_balance(conn, wallet.id, amount.clone()).await?;
    let internal_transaction_id =
        create_internal_transaction(conn, wallet.id, amount.clone()).await?;
    let transaction_id = create_wallet_credit_transaction(conn, internal_transaction_id).await?;
    let credit_record = reward_wallet_credit_record_repository::create_reward_wallet_credit_record(
        conn,
        NewRewardWalletCreditRecord {
            reward_candidate_id: candidate.id,
            wallet_id: wallet.id,
            transaction_id,
            internal_transaction_id,
        },
    )
    .await
    .map_err(map_diesel_error)?;
    let updated = mark_candidate_wallet_credited(conn, candidate.id).await?;
    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id,
            event_type: REWARD_AUDIT_EVENT_WALLET_CREDITED.to_string(),
            from_status: Some(candidate.status.clone()),
            to_status: updated.status,
            reason: None,
            metadata: serde_json::json!({
                "wallet_id": wallet.id,
                "credit_record_id": credit_record.id,
                "transaction_id": transaction_id,
                "internal_transaction_id": internal_transaction_id,
            }),
        },
    )
    .await
    .map_err(map_diesel_error)?;

    Ok(RewardWalletCreditOutput {
        candidate_id: candidate.id,
        wallet_id: wallet.id,
        credit_record_id: Some(credit_record.id),
        transaction_id: Some(transaction_id),
        internal_transaction_id: Some(internal_transaction_id),
        amount,
        credited: true,
    })
}

async fn mark_candidate_wallet_credited(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardCandidate, RewardWalletCreditError> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(RewardCandidateStatus::WalletCredited.as_str()),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}
