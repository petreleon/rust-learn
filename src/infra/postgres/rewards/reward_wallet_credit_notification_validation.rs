use bigdecimal::BigDecimal;

use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationError;
use crate::domain::rewards::candidate::lifecycle;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn approved_positive_amount(
    candidate: &RewardCandidate,
) -> Result<BigDecimal, RewardWalletCreditNotificationError> {
    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardWalletCreditNotificationError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardWalletCreditNotificationError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }
    Ok(amount)
}

pub(super) fn ensure_notification_can_be_inspected(
    candidate: &RewardCandidate,
    allow_reconciliation_repair: bool,
) -> Result<(), RewardWalletCreditNotificationError> {
    if can_inspect_notification(candidate, allow_reconciliation_repair) {
        Ok(())
    } else {
        Err(RewardWalletCreditNotificationError::InvalidStatus(
            "reward candidate must be wallet credited before notification".to_string(),
        ))
    }
}

pub(super) fn missing_notification_target_status(
    candidate: &RewardCandidate,
    allow_reconciliation_repair: bool,
) -> Result<RewardCandidateStatus, RewardWalletCreditNotificationError> {
    let status = RewardCandidateStatus::parse(&candidate.status).map_err(|_| {
        RewardWalletCreditNotificationError::InvalidStatus(
            "notified candidate is missing its notification reference".to_string(),
        )
    })?;
    lifecycle::wallet_credit_notification_target_status(status, allow_reconciliation_repair)
        .ok_or_else(|| {
            RewardWalletCreditNotificationError::InvalidStatus(
                "notified candidate is missing its notification reference".to_string(),
            )
        })
}

fn can_inspect_notification(
    candidate: &RewardCandidate,
    allow_reconciliation_repair: bool,
) -> bool {
    RewardCandidateStatus::parse(&candidate.status)
        .map(|status| {
            lifecycle::can_inspect_wallet_credit_notification(status, allow_reconciliation_repair)
        })
        .unwrap_or(false)
}
