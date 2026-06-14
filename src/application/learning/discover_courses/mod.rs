mod error;
mod handler;
mod output;
mod query;
mod service;
mod store;

#[cfg(test)]
mod tests;

pub use error::CourseDiscoveryError;
pub use handler::discover_courses;
pub use output::{CourseDiscoveryCourseOutput, CourseDiscoveryOutput};
pub use query::CourseDiscoveryQuery;
pub use service::CourseDiscoveryUseCase;
pub use store::CourseDiscoveryStore;
