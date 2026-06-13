// TODO(level-2): delete this compatibility wrapper when callers import
// `http::operations` directly.
pub use crate::http::operations::configure_routes as configure_health_routes;
pub use crate::http::operations::health_scope;
