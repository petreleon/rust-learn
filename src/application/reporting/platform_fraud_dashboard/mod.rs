mod error;
mod handler;
mod output;
mod service;
pub mod store;

pub use error::PlatformFraudDashboardError;
pub use handler::load_platform_fraud_dashboard;
pub use output::{
    FraudBlockDashboardRowOutput, FraudBlockScopeSummaryOutput, PlatformFraudDashboardOutput,
};
pub use service::PlatformFraudDashboardUseCase;
