mod error;
mod handler;
mod output;
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
pub use service::PlatformRewardDashboardUseCase;
pub(crate) use summary::record_reward_candidate_status_count;
