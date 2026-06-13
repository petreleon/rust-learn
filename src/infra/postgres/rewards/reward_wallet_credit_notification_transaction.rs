use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::notify_wallet_credit::{
    RewardWalletCreditNotification, RewardWalletCreditNotificationError,
    RewardWalletCreditNotificationOutput,
};
use crate::db::schema::reward_candidates;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::rewards::reward_wallet_credit_notification_mappers::{
    map_diesel_error, RewardWalletCreditNotificationTransactionError,
};
use crate::infra::postgres::rewards::reward_wallet_credit_notification_notifications::create_wallet_credit_notification;
use crate::infra::postgres::rewards::reward_wallet_credit_notification_validation::{
    approved_positive_amount, ensure_missing_notification_can_be_created,
    ensure_notification_can_be_inspected,
};
use crate::models::reward_audit_event::{
    NewRewardAuditEvent, REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED,
};
use crate::models::reward_candidate::RewardCandidate;
use crate::repositories::{
    reward_audit_event_repository, reward_candidate_repository,
    reward_wallet_credit_record_repository,
};

pub(super) async fn notify_reward_wallet_credit(
    conn: &mut AsyncPgConnection,
    notification: RewardWalletCreditNotification,
) -> Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationTransactionError> {
    let candidate =
        reward_candidate_repository::find_candidate(conn, notification.candidate_id).await?;
    notify_reward_wallet_credit_for_candidate(
        conn,
        &candidate,
        notification.allow_reconciliation_repair,
        notification.actor_user_id,
    )
    .await
    .map_err(RewardWalletCreditNotificationTransactionError::from)
}

pub(crate) async fn notify_reward_wallet_credit_for_candidate(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_repair: bool,
    actor_user_id: Option<i32>,
) -> Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError> {
    ensure_notification_can_be_inspected(candidate, allow_reconciliation_repair)?;
    let amount = approved_positive_amount(candidate)?;
    let credit_record =
        reward_wallet_credit_record_repository::find_reward_wallet_credit_record_by_candidate(
            conn,
            candidate.id,
        )
        .await
        .map_err(map_diesel_error)?
        .ok_or_else(|| {
            RewardWalletCreditNotificationError::InvalidStatus(
                "wallet credited candidate is missing a reward wallet credit record".to_string(),
            )
        })?;

    if let Some(notification_id) = credit_record.notification_id {
        return Ok(RewardWalletCreditNotificationOutput {
            candidate_id: candidate.id,
            wallet_id: credit_record.wallet_id,
            notification_id: Some(notification_id),
            transaction_id: credit_record.transaction_id,
            amount,
            notified: false,
        });
    }

    ensure_missing_notification_can_be_created(candidate, allow_reconciliation_repair)?;
    let notification_id = create_wallet_credit_notification(
        conn,
        candidate.student_user_id,
        candidate.course_id,
        &amount,
        credit_record.wallet_id,
        credit_record.transaction_id,
    )
    .await?;
    reward_wallet_credit_record_repository::mark_reward_wallet_credit_record_notified(
        conn,
        credit_record.id,
        notification_id,
    )
    .await
    .map_err(map_diesel_error)?;
    let updated = mark_candidate_notified(conn, candidate.id).await?;
    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id,
            event_type: REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED.to_string(),
            from_status: Some(candidate.status.clone()),
            to_status: updated.status,
            reason: None,
            metadata: serde_json::json!({
                "wallet_id": credit_record.wallet_id,
                "notification_id": notification_id,
                "transaction_id": credit_record.transaction_id,
            }),
        },
    )
    .await
    .map_err(map_diesel_error)?;

    Ok(RewardWalletCreditNotificationOutput {
        candidate_id: candidate.id,
        wallet_id: credit_record.wallet_id,
        notification_id: Some(notification_id),
        transaction_id: credit_record.transaction_id,
        amount,
        notified: true,
    })
}

async fn mark_candidate_notified(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardCandidate, RewardWalletCreditNotificationError> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(RewardCandidateStatus::Notified.as_str()),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}
