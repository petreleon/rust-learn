#[derive(Debug, PartialEq, Eq)]
pub enum LearnerCourseCatalogError {
    PermissionDenied(String),
    NotFound,
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TeacherCourseDashboardError {
    PermissionDenied(String),
    NotFound,
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrganizationCourseListError {
    PermissionDenied(String),
    NotFound,
    Database(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct CourseLifecycleUpdateRequest {
    pub status: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CourseLifecycleError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CourseCreationError {
    PermissionDenied(String),
    Database(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CourseUpdateError {
    PermissionDenied(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for CourseUpdateError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => CourseUpdateError::NotFound,
            other => CourseUpdateError::Database(other.to_string()),
        }
    }
}

impl From<diesel::result::Error> for CourseCreationError {
    fn from(error: diesel::result::Error) -> Self {
        CourseCreationError::Database(error.to_string())
    }
}

impl From<diesel::result::Error> for CourseLifecycleError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => CourseLifecycleError::NotFound,
            other => CourseLifecycleError::Database(other.to_string()),
        }
    }
}
