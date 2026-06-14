#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformFraudDashboardError {
    Connection(String),
    Database(String),
}
