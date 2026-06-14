use crate::domain::teacher_applications::scope::TeacherApplicationScopeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationSubmitError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    Connection(String),
    Database(String),
}

impl From<TeacherApplicationScopeError> for TeacherApplicationSubmitError {
    fn from(error: TeacherApplicationScopeError) -> Self {
        match error {
            TeacherApplicationScopeError::InvalidScope(message)
            | TeacherApplicationScopeError::MissingOrganizationTarget(message)
            | TeacherApplicationScopeError::MissingCourseTarget(message) => {
                Self::InvalidInput(message)
            }
        }
    }
}
