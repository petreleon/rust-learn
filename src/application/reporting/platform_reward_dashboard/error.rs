#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformRewardDashboardError {
    Connection(String),
    Database(String),
}
