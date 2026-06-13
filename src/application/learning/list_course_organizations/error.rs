#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseOrganizationReadError {
    Connection(String),
    Database(String),
}
