#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationManagementError {
    NotFound,
    Connection(String),
    Database(String),
}
