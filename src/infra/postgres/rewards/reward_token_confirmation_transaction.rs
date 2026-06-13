use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmation, RewardTokenConfirmationError, RewardTokenConfirmationOutput,
};
use crate::db::schema::reward_candidates;
use crate::domain::rewards::audit::RewardAuditEventType;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::candidate::transition::{apply_transition, TransitionAction};
use crate::infra::postgres::rewards::reward_token_confirmation_external_transactions::record_external_reward_transaction;
use crate::infra::postgres::rewards::reward_token_confirmation_mappers::RewardTokenConfirmationTransactionError;
use crate::models::reward_audit_event::NewRewardAuditEvent;
use crate::models::reward_candidate::RewardCandidate;
use crate::models::reward_payout_record::NewRewardPayoutRecord;
use crate::repositories::{
    reward_audit_event_repository, reward_candidate_repository, reward_payout_record_repository,
};

pub(super) async fn record_reward_token_confirmation(
    conn: &mut AsyncPgConnection,
    confirmation: RewardTokenConfirmation,
) -> Result<RewardTokenConfirmationOutput, RewardTokenConfirmationTransactionError> {
    if let Some(existing_record) =
        reward_payout_record_repository::find_reward_payout_record_by_candidate(
            conn,
            confirmation.candidate_id,
        )
        .await?
    {
        return Ok(RewardTokenConfirmationOutput {
            candidate_id: confirmation.candidate_id,
            transaction_id: existing_record.transaction_id,
            external_transaction_id: existing_record.external_transaction_id,
            payout_record_id: existing_record.id,
            inserted_external_transaction: false,
        });
    }

    let candidate =
        reward_candidate_repository::find_candidate(conn, confirmation.candidate_id).await?;
    let to_status = confirmed_token_status(&candidate)?;
    let recorded = record_external_reward_transaction(
        conn,
        &confirmation.command,
        &confirmation.transaction_type,
    )
    .await?;
    let payout_record = reward_payout_record_repository::create_reward_payout_record(
        conn,
        NewRewardPayoutRecord {
            reward_candidate_id: candidate.id,
            transaction_id: recorded.transaction_id,
            external_transaction_id: recorded.external_transaction_id,
        },
    )
    .await?;
    let updated = mark_candidate_token_confirmed(conn, candidate.id, to_status.as_str()).await?;
    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id: confirmation.actor_user_id,
            event_type: RewardAuditEventType::TokenConfirmed.as_str().to_string(),
            from_status: Some(candidate.status.clone()),
            to_status: updated.status,
            reason: None,
            metadata: serde_json::json!({
                "transaction_id": recorded.transaction_id,
                "external_transaction_id": recorded.external_transaction_id,
                "payout_record_id": payout_record.id,
                "inserted_external_transaction": recorded.inserted_external_transaction,
            }),
        },
    )
    .await?;

    Ok(RewardTokenConfirmationOutput {
        candidate_id: candidate.id,
        transaction_id: recorded.transaction_id,
        external_transaction_id: recorded.external_transaction_id,
        payout_record_id: payout_record.id,
        inserted_external_transaction: recorded.inserted_external_transaction,
    })
}

fn confirmed_token_status(
    candidate: &RewardCandidate,
) -> Result<RewardCandidateStatus, RewardTokenConfirmationError> {
    let from_status = RewardCandidateStatus::parse(&candidate.status).map_err(|_| {
        RewardTokenConfirmationError::InvalidStatus(
            "reward candidate must be token pending before token confirmation".to_string(),
        )
    })?;
    apply_transition(from_status, TransitionAction::ConfirmToken).map_err(|_| {
        RewardTokenConfirmationError::InvalidStatus(
            "reward candidate must be token pending before token confirmation".to_string(),
        )
    })
}

async fn mark_candidate_token_confirmed(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    to_status: &str,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(to_status),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}
