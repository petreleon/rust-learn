use crate::application::organizations::get_organization_dashboard::OrganizationDashboardRewardSummaryOutput;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

pub(crate) fn record_organization_dashboard_reward_status(
    summary: &mut OrganizationDashboardRewardSummaryOutput,
    status: RewardCandidateStatus,
) {
    match status {
        RewardCandidateStatus::Failed => {
            summary.failed_count += 1;
        }
        RewardCandidateStatus::NeedsReconciliation => {
            summary.needs_reconciliation_count += 1;
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus as Status;

    fn summary() -> OrganizationDashboardRewardSummaryOutput {
        OrganizationDashboardRewardSummaryOutput {
            available: true,
            missing_permissions: vec![],
            reward_candidate_count: 3,
            approved_reward_count: 2,
            approved_amount_total: "25".to_string(),
            failed_count: 0,
            needs_reconciliation_count: 0,
        }
    }

    #[test]
    fn records_reward_attention_statuses_for_organization_dashboard() {
        let mut summary = summary();

        record_organization_dashboard_reward_status(&mut summary, Status::Failed);
        record_organization_dashboard_reward_status(&mut summary, Status::NeedsReconciliation);
        record_organization_dashboard_reward_status(&mut summary, Status::Failed);

        assert_eq!(summary.failed_count, 2);
        assert_eq!(summary.needs_reconciliation_count, 1);
    }

    #[test]
    fn ignores_non_attention_reward_statuses() {
        let mut summary = summary();

        for status in [
            Status::PendingTeacherApproval,
            Status::TeacherApproved,
            Status::AmountApproved,
            Status::WalletCredited,
            Status::Completed,
        ] {
            record_organization_dashboard_reward_status(&mut summary, status);
        }

        assert_eq!(summary.failed_count, 0);
        assert_eq!(summary.needs_reconciliation_count, 0);
    }
}
