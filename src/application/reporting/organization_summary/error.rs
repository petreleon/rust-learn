#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationSummaryError {
    NotFound,
    Connection(String),
    Database(String),
}
