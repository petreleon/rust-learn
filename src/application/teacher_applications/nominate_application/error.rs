use crate::domain::teacher_applications::scope::TeacherApplicationScopeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationNominationError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    NotFound,
    Connection(String),
    Database(String),
}

impl From<TeacherApplicationScopeError> for TeacherApplicationNominationError {
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
