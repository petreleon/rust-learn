#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationRewardDashboardError {
    NotFound,
    Connection(String),
    Database(String),
}
