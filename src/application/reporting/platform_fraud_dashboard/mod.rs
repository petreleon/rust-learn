mod aggregation;
mod error;
mod handler;
mod output;
mod service;
pub mod store;

pub(crate) use aggregation::{platform_fraud_dashboard_from_facts, FraudBlockDashboardFact};
pub use error::PlatformFraudDashboardError;
pub use handler::load_platform_fraud_dashboard;
pub use output::{
    FraudBlockDashboardRowOutput, FraudBlockScopeSummaryOutput, PlatformFraudDashboardOutput,
};
pub use service::PlatformFraudDashboardUseCase;
