#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseEnrollmentError {
    PermissionDenied(String),
    InvalidStatus(String),
    CourseCapacityFull { max: i32, current: i64 },
    NotFound,
    Connection(String),
    Database(String),
}
