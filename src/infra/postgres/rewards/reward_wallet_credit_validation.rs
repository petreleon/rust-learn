use bigdecimal::BigDecimal;
use diesel_async::AsyncPgConnection;

use crate::application::rewards::credit_wallet::RewardWalletCreditError;
use crate::domain::rewards::candidate::lifecycle;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::candidate::transition;
use crate::infra::postgres::rewards::reward_wallet_credit_policy::reward_policy_is_off_chain;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn approved_positive_amount(
    candidate: &RewardCandidate,
) -> Result<BigDecimal, RewardWalletCreditError> {
    let amount = candidate.approved_amount.clone().ok_or_else(|| {
        RewardWalletCreditError::InvalidInput(
            "reward candidate must have an approved amount".to_string(),
        )
    })?;
    if amount <= BigDecimal::from(0) {
        return Err(RewardWalletCreditError::InvalidInput(
            "approved reward amount must be positive".to_string(),
        ));
    }
    Ok(amount)
}

pub(super) fn reject_already_credited_without_record(
    candidate: &RewardCandidate,
) -> Result<(), RewardWalletCreditError> {
    match RewardCandidateStatus::parse(&candidate.status) {
        Ok(status) if lifecycle::requires_wallet_credit_record(status) => {
            Err(RewardWalletCreditError::InvalidStatus(
                "wallet credited candidate is missing a reward wallet credit record".to_string(),
            ))
        }
        _ => Ok(()),
    }
}

pub(super) async fn wallet_credit_target_status(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
    allow_reconciliation_credit: bool,
) -> Result<RewardCandidateStatus, RewardWalletCreditError> {
    let status = RewardCandidateStatus::parse(&candidate.status).map_err(|_| {
        RewardWalletCreditError::InvalidStatus(
            "wallet credit requires token confirmation unless the reward policy is off-chain"
                .to_string(),
        )
    })?;

    if status == RewardCandidateStatus::NeedsReconciliation && !allow_reconciliation_credit {
        return Err(RewardWalletCreditError::InvalidStatus(
            "needs reconciliation candidate requires confirmed payout evidence before wallet credit"
                .to_string(),
        ));
    }

    if status == RewardCandidateStatus::AmountApproved
        && !reward_policy_is_off_chain(conn, candidate.course_id, &candidate.event_type).await?
    {
        return Err(token_confirmation_required_error());
    }

    transition::credit_wallet(status).map_err(|_| token_confirmation_required_error())
}

fn token_confirmation_required_error() -> RewardWalletCreditError {
    RewardWalletCreditError::InvalidStatus(
        "wallet credit requires token confirmation unless the reward policy is off-chain"
            .to_string(),
    )
}
