#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationDashboardError {
    PermissionDenied,
    NotFound,
    Connection(String),
    Database(String),
    Reporting(String),
}
