#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseDiscoveryError {
    Connection(String),
    Database(String),
}
