#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseRoleAssignmentError {
    HierarchyViolation,
    NotFound,
    Connection(String),
    Database(String),
}
