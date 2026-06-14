use crate::application::reporting::platform_reward_dashboard::RewardCandidateDashboardSummaryOutput;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub(crate) fn record_reward_candidate_status_count(
    summary: &mut RewardCandidateDashboardSummaryOutput,
    status: RewardCandidateStatus,
    count: i64,
) {
    match status {
        RewardCandidateStatus::PendingTeacherApproval => {
            summary.pending_teacher_approval = count;
        }
        RewardCandidateStatus::TeacherApproved => {
            summary.teacher_approved = count;
        }
        RewardCandidateStatus::TeacherRejected => {
            summary.teacher_rejected = count;
        }
        RewardCandidateStatus::AmountApproved => {
            summary.amount_approved = count;
        }
        RewardCandidateStatus::AmountRejected => {
            summary.amount_rejected = count;
        }
        RewardCandidateStatus::Adjusted => {}
        RewardCandidateStatus::TokenPending => {
            summary.token_pending = count;
        }
        RewardCandidateStatus::TokenConfirmed => {
            summary.token_confirmed = count;
        }
        RewardCandidateStatus::WalletCredited => {
            summary.wallet_credited = count;
        }
        RewardCandidateStatus::Notified => {
            summary.notified = count;
        }
        RewardCandidateStatus::Completed => {
            summary.completed = count;
        }
        RewardCandidateStatus::NeedsReconciliation => {
            summary.needs_reconciliation = count;
        }
        RewardCandidateStatus::Failed => {
            summary.failed = count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus as Status;

    #[test]
    fn records_reward_candidate_counts_by_status() {
        let mut summary = RewardCandidateDashboardSummaryOutput::default();

        for (index, status) in [
            Status::PendingTeacherApproval,
            Status::TeacherApproved,
            Status::TeacherRejected,
            Status::AmountApproved,
            Status::AmountRejected,
            Status::TokenPending,
            Status::TokenConfirmed,
            Status::WalletCredited,
            Status::Notified,
            Status::Completed,
            Status::NeedsReconciliation,
            Status::Failed,
        ]
        .into_iter()
        .enumerate()
        {
            record_reward_candidate_status_count(&mut summary, status, index as i64 + 1);
        }

        assert_eq!(summary.pending_teacher_approval, 1);
        assert_eq!(summary.teacher_approved, 2);
        assert_eq!(summary.teacher_rejected, 3);
        assert_eq!(summary.amount_approved, 4);
        assert_eq!(summary.amount_rejected, 5);
        assert_eq!(summary.token_pending, 6);
        assert_eq!(summary.token_confirmed, 7);
        assert_eq!(summary.wallet_credited, 8);
        assert_eq!(summary.notified, 9);
        assert_eq!(summary.completed, 10);
        assert_eq!(summary.needs_reconciliation, 11);
        assert_eq!(summary.failed, 12);
    }

    #[test]
    fn adjusted_status_does_not_have_a_dashboard_bucket() {
        let mut summary = RewardCandidateDashboardSummaryOutput {
            total: 9,
            ..RewardCandidateDashboardSummaryOutput::default()
        };

        record_reward_candidate_status_count(&mut summary, Status::Adjusted, 4);

        assert_eq!(
            summary,
            RewardCandidateDashboardSummaryOutput {
                total: 9,
                ..RewardCandidateDashboardSummaryOutput::default()
            }
        );
    }
}
