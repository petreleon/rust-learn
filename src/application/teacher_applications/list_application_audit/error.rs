#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationAuditError {
    PermissionDenied(String),
    Connection(String),
    Database(String),
}
