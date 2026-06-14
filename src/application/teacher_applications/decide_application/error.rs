use crate::domain::teacher_applications::status::TeacherApplicationStatusError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationDecisionError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    NotFound,
    Connection(String),
    Database(String),
}

impl From<TeacherApplicationStatusError> for TeacherApplicationDecisionError {
    fn from(error: TeacherApplicationStatusError) -> Self {
        match error {
            TeacherApplicationStatusError::InvalidStatus(message) => Self::InvalidInput(message),
        }
    }
}
