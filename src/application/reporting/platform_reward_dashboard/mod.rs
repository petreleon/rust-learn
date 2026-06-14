mod dashboard;
mod error;
mod handler;
mod output;
mod reconciliation;
mod rows;
mod service;
pub mod store;
mod summary;

pub(crate) use dashboard::{platform_reward_dashboard_output, PlatformRewardDashboardFact};
pub use error::PlatformRewardDashboardError;
pub use handler::load_platform_reward_dashboard;
pub use output::{
    PlatformRewardDashboardOutput, RewardCandidateDashboardRowOutput,
    RewardCandidateDashboardSummaryOutput, RewardExecutionFailureRowOutput,
    RewardReconciliationMismatchRowOutput, TeacherApplicationDashboardSummaryOutput,
};
pub(crate) use reconciliation::{
    classify_reward_reconciliation_mismatch, reconciliation_mismatch_candidate_statuses,
    RewardReconciliationMismatchFacts,
};
pub(crate) use rows::{
    reward_candidate_dashboard_row, reward_execution_failure_row,
    reward_reconciliation_mismatch_row, RewardCandidateDashboardRowFact,
    RewardExecutionFailureRowFact, RewardReconciliationMismatchRowFact,
};
pub use service::PlatformRewardDashboardUseCase;
pub(crate) use summary::record_reward_candidate_status_count;
