use crate::domain::teacher_applications::status::TeacherApplicationStatusError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationListError {
    PermissionDenied(String),
    InvalidInput(String),
    Connection(String),
    Database(String),
}

impl From<TeacherApplicationStatusError> for TeacherApplicationListError {
    fn from(error: TeacherApplicationStatusError) -> Self {
        match error {
            TeacherApplicationStatusError::InvalidStatus(message) => Self::InvalidInput(message),
        }
    }
}
