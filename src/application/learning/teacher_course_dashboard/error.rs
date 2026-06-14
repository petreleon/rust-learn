#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherCourseDashboardError {
    PermissionDenied(String),
    NotFound,
    Connection(String),
    Database(String),
}
