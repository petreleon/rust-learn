pub mod access_control;
pub mod content;
pub mod errors;
pub mod extractors;
pub mod identity;
pub mod kyc;
pub mod learning;
pub mod notifications;
pub mod operations;
pub mod organizations;
pub mod reporting;
pub mod request_params;
pub mod rewards;
mod routes;
pub mod teacher_applications;
pub mod wallet;

pub use routes::api_scope;
