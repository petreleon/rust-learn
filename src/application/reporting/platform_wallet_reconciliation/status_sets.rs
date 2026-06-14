use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub(crate) fn needs_reconciliation_candidate_statuses() -> [RewardCandidateStatus; 1] {
    [RewardCandidateStatus::NeedsReconciliation]
}

pub(crate) fn missing_credit_record_candidate_statuses() -> [RewardCandidateStatus; 3] {
    [
        RewardCandidateStatus::WalletCredited,
        RewardCandidateStatus::Notified,
        RewardCandidateStatus::Completed,
    ]
}

pub(crate) fn missing_notification_record_candidate_statuses() -> [RewardCandidateStatus; 2] {
    [
        RewardCandidateStatus::Notified,
        RewardCandidateStatus::Completed,
    ]
}

pub(crate) fn missing_payout_record_candidate_statuses() -> [RewardCandidateStatus; 1] {
    [RewardCandidateStatus::TokenConfirmed]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus as Status;

    #[test]
    fn exposes_wallet_reconciliation_attention_statuses() {
        assert_eq!(
            needs_reconciliation_candidate_statuses(),
            [Status::NeedsReconciliation]
        );
    }

    #[test]
    fn exposes_missing_record_status_sets() {
        assert_eq!(
            missing_credit_record_candidate_statuses(),
            [Status::WalletCredited, Status::Notified, Status::Completed]
        );
        assert_eq!(
            missing_notification_record_candidate_statuses(),
            [Status::Notified, Status::Completed]
        );
        assert_eq!(
            missing_payout_record_candidate_statuses(),
            [Status::TokenConfirmed]
        );
    }
}
