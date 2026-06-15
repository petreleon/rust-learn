use diesel_async::AsyncPgConnection;

use crate::application::rewards::credit_wallet::{
    RewardWalletCredit, RewardWalletCreditError, RewardWalletCreditOutput,
};
use crate::domain::rewards::audit::RewardAuditEventType;
use crate::infra::postgres::rewards::reward_audit_records::create_reward_audit_event;
use crate::infra::postgres::rewards::reward_candidate_records::{
    find_candidate, update_candidate_status,
};
use crate::infra::postgres::rewards::reward_wallet_credit_mappers::{
    map_diesel_error, RewardWalletCreditTransactionError,
};
use crate::infra::postgres::rewards::reward_wallet_credit_records::{
    create_reward_wallet_credit_record, find_reward_wallet_credit_record_by_candidate,
};
use crate::infra::postgres::rewards::reward_wallet_credit_transactions::{
    create_internal_transaction, create_wallet_credit_transaction,
};
use crate::infra::postgres::rewards::reward_wallet_credit_validation::{
    approved_positive_amount, reject_already_credited_without_record, wallet_credit_target_status,
};
use crate::infra::postgres::rewards::reward_wallet_credit_wallets::{
    credit_wallet_balance, link_user_wallet,
};
use crate::models::reward_audit_event::NewRewardAuditEvent;
use crate::models::reward_candidate::RewardCandidate;
use crate::models::reward_wallet_credit_record::NewRewardWalletCreditRecord;

pub(super) async fn credit_reward_wallet(
    conn: &mut AsyncPgConnection,
    credit: RewardWalletCredit,
) -> Result<RewardWalletCreditOutput, RewardWalletCreditTransactionError> {
    let candidate = find_candidate(conn, credit.candidate_id).await?;
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
    let existing_record = find_reward_wallet_credit_record_by_candidate(conn, candidate.id)
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
    let target_status =
        wallet_credit_target_status(conn, candidate, allow_reconciliation_credit).await?;

    let wallet = link_user_wallet(conn, candidate.student_user_id).await?;
    let wallet = credit_wallet_balance(conn, wallet.id, amount.clone()).await?;
    let internal_transaction_id =
        create_internal_transaction(conn, wallet.id, amount.clone()).await?;
    let transaction_id = create_wallet_credit_transaction(conn, internal_transaction_id).await?;
    let credit_record = create_reward_wallet_credit_record(
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
    let updated = update_candidate_status(
        conn,
        candidate.id,
        target_status.as_str(),
        chrono::Utc::now(),
    )
    .await
    .map_err(map_diesel_error)?;
    create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id,
            event_type: RewardAuditEventType::WalletCredited.as_str().to_string(),
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
