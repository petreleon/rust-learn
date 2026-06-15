mod error;
mod handler;
mod output;
mod service;
pub mod store;
mod summary;

pub use error::PlatformSummaryError;
pub use handler::load_platform_summary;
pub use output::PlatformSummaryOutput;
pub use service::PlatformSummaryUseCase;
pub(crate) use summary::{platform_summary_output, PlatformSummaryFact};
