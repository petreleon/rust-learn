mod error;
mod handler;
mod output;
mod reconciliation;
mod service;
pub mod store;
mod summary;

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
pub use service::PlatformRewardDashboardUseCase;
pub(crate) use summary::record_reward_candidate_status_count;
