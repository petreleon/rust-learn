use diesel_async::AsyncPgConnection;

use crate::application::rewards::reconcile_candidate::{
    RewardReconciliation, RewardReconciliationError, RewardReconciliationOutput,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::rewards::reward_candidate_records::find_candidate;
use crate::infra::postgres::rewards::reward_payout_records::find_reward_payout_record_by_candidate;
use crate::infra::postgres::rewards::reward_reconciliation_audit::{
    create_reconciliation_audit_event, ReconciliationOutcome,
};
use crate::infra::postgres::rewards::reward_reconciliation_links::{
    ensure_external_transaction_link, ensure_internal_transaction_link,
};
use crate::infra::postgres::rewards::reward_reconciliation_mappers::{
    map_diesel_error, map_wallet_credit_error, map_wallet_notification_error,
    RewardReconciliationTransactionError,
};
use crate::infra::postgres::rewards::reward_reconciliation_validation::{
    ensure_candidate_reconcilable, should_create_reconciliation_wallet_credit,
};
use crate::infra::postgres::rewards::reward_wallet_credit_records::find_reward_wallet_credit_record_by_candidate;

pub(super) async fn reconcile_reward_candidate(
    conn: &mut AsyncPgConnection,
    reconciliation: RewardReconciliation,
) -> Result<RewardReconciliationOutput, RewardReconciliationTransactionError> {
    reconcile_candidate_transaction(conn, reconciliation)
        .await
        .map_err(RewardReconciliationTransactionError::from)
}

async fn reconcile_candidate_transaction(
    conn: &mut AsyncPgConnection,
    reconciliation: RewardReconciliation,
) -> Result<RewardReconciliationOutput, RewardReconciliationError> {
    let mut candidate = find_candidate(conn, reconciliation.candidate_id)
        .await
        .map_err(map_diesel_error)?;
    ensure_candidate_reconcilable(&candidate)?;
    let initial_status = candidate.status.clone();

    let payout_record = find_reward_payout_record_by_candidate(conn, candidate.id)
        .await
        .map_err(map_diesel_error)?;
    let external_transaction_link_repaired = match payout_record.as_ref() {
        Some(record) => {
            ensure_external_transaction_link(
                conn,
                record.transaction_id,
                record.external_transaction_id,
            )
            .await?
        }
        None => false,
    };

    let mut credit_record = find_reward_wallet_credit_record_by_candidate(conn, candidate.id)
        .await
        .map_err(map_diesel_error)?;
    let mut wallet_credit_created = false;
    if credit_record.is_none() && should_create_reconciliation_wallet_credit(&candidate) {
        wallet_credit_created =
            super::reward_wallet_credit_transaction::credit_reward_wallet_for_candidate(
                conn,
                &candidate,
                payout_record.is_some(),
                reconciliation.actor_user_id,
            )
            .await
            .map_err(map_wallet_credit_error)?
            .credited;
        candidate = find_candidate(conn, candidate.id)
            .await
            .map_err(map_diesel_error)?;
        credit_record = find_reward_wallet_credit_record_by_candidate(conn, candidate.id)
            .await
            .map_err(map_diesel_error)?;
    }

    let internal_transaction_link_repaired = match credit_record.as_ref() {
        Some(record) => {
            ensure_internal_transaction_link(
                conn,
                record.transaction_id,
                record.internal_transaction_id,
            )
            .await?
        }
        None => false,
    };

    let mut notification_created = false;
    if credit_record.is_some() {
        let notification_result =
            super::reward_wallet_credit_notification_transaction::notify_reward_wallet_credit_for_candidate(
                conn,
                &candidate,
                true,
                reconciliation.actor_user_id,
            )
            .await
            .map_err(map_wallet_notification_error)?;
        notification_created = notification_result.notified;
        candidate = find_candidate(conn, candidate.id)
            .await
            .map_err(map_diesel_error)?;
    }

    let outcome = ReconciliationOutcome {
        wallet_credit_created,
        notification_created,
        external_transaction_link_repaired,
        internal_transaction_link_repaired,
    };
    create_reconciliation_audit_event(
        conn,
        candidate.id,
        reconciliation.actor_user_id,
        initial_status,
        candidate.status.clone(),
        &outcome,
    )
    .await?;

    let final_status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| RewardReconciliationError::InvalidStatus(error.to_string()))?;

    Ok(RewardReconciliationOutput {
        candidate_id: candidate.id,
        wallet_credit_created,
        notification_created,
        external_transaction_link_repaired,
        internal_transaction_link_repaired,
        final_status,
    })
}
