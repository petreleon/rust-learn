use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardOutput, RewardCandidateDashboardRowOutput,
    RewardCandidateDashboardSummaryOutput, RewardExecutionFailureRowOutput,
    RewardReconciliationMismatchRowOutput, TeacherApplicationDashboardSummaryOutput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlatformRewardDashboardFact {
    pub teacher_applications: TeacherApplicationDashboardSummaryOutput,
    pub reward_candidates: RewardCandidateDashboardSummaryOutput,
    pub pending_amount_approval_count: i64,
    pub pending_amount_approvals: Vec<RewardCandidateDashboardRowOutput>,
    pub payout_failure_count: i64,
    pub payout_failures: Vec<RewardExecutionFailureRowOutput>,
    pub reconciliation_mismatches: Vec<RewardReconciliationMismatchRowOutput>,
}

pub(crate) fn platform_reward_dashboard_output(
    fact: PlatformRewardDashboardFact,
) -> PlatformRewardDashboardOutput {
    PlatformRewardDashboardOutput {
        teacher_applications: fact.teacher_applications,
        reward_candidates: fact.reward_candidates,
        pending_amount_approval_count: fact.pending_amount_approval_count,
        pending_amount_approvals: fact.pending_amount_approvals,
        payout_failure_count: fact.payout_failure_count,
        payout_failures: fact.payout_failures,
        reconciliation_mismatch_count: fact.reconciliation_mismatches.len() as i64,
        reconciliation_mismatches: fact.reconciliation_mismatches,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::reporting::platform_reward_dashboard::RewardReconciliationMismatchType;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;

    #[test]
    fn builds_dashboard_and_counts_reconciliation_mismatches() {
        let output = platform_reward_dashboard_output(PlatformRewardDashboardFact {
            teacher_applications: TeacherApplicationDashboardSummaryOutput {
                submitted: 2,
                ..Default::default()
            },
            reward_candidates: RewardCandidateDashboardSummaryOutput {
                teacher_approved: 3,
                ..Default::default()
            },
            pending_amount_approval_count: 4,
            pending_amount_approvals: Vec::new(),
            payout_failure_count: 5,
            payout_failures: Vec::new(),
            reconciliation_mismatches: vec![RewardReconciliationMismatchRowOutput {
                reward_candidate_id: 6,
                course_id: 7,
                student_user_id: 8,
                status: RewardCandidateStatus::TokenConfirmed,
                mismatch_type: RewardReconciliationMismatchType::NeedsPayoutRecord,
                approved_amount: None,
                updated_at: chrono::Utc::now(),
            }],
        });

        assert_eq!(output.teacher_applications.submitted, 2);
        assert_eq!(output.reward_candidates.teacher_approved, 3);
        assert_eq!(output.reconciliation_mismatch_count, 1);
    }
}
