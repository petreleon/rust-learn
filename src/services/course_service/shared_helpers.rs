use super::errors::{
    LearnerCourseCatalogError, OrganizationCourseListError, TeacherCourseDashboardError,
};
use super::LIKE_ESCAPE_CHAR;

pub(super) fn course_title_search_pattern(search: &str) -> String {
    let mut escaped = String::with_capacity(search.len());
    for ch in search.chars() {
        match ch {
            LIKE_ESCAPE_CHAR | '%' | '_' => {
                escaped.push(LIKE_ESCAPE_CHAR);
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }

    format!("%{}%", escaped)
}

pub(super) fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

impl From<diesel::result::Error> for LearnerCourseCatalogError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
            other => LearnerCourseCatalogError::Database(other.to_string()),
        }
    }
}

impl From<diesel::result::Error> for TeacherCourseDashboardError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
            other => TeacherCourseDashboardError::Database(other.to_string()),
        }
    }
}

impl From<LearnerCourseCatalogError> for TeacherCourseDashboardError {
    fn from(error: LearnerCourseCatalogError) -> Self {
        match error {
            LearnerCourseCatalogError::Database(message) => {
                TeacherCourseDashboardError::Database(message)
            }
            LearnerCourseCatalogError::NotFound => {
                TeacherCourseDashboardError::Database("course not found".to_string())
            }
            LearnerCourseCatalogError::PermissionDenied(permission) => {
                TeacherCourseDashboardError::Database(format!(
                    "unexpected permission error while building teacher dashboard: {}",
                    permission
                ))
            }
        }
    }
}

impl From<diesel::result::Error> for OrganizationCourseListError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => OrganizationCourseListError::NotFound,
            other => OrganizationCourseListError::Database(other.to_string()),
        }
    }
}

impl From<LearnerCourseCatalogError> for OrganizationCourseListError {
    fn from(error: LearnerCourseCatalogError) -> Self {
        match error {
            LearnerCourseCatalogError::Database(message) => {
                OrganizationCourseListError::Database(message)
            }
            LearnerCourseCatalogError::NotFound => {
                OrganizationCourseListError::Database("course not found".to_string())
            }
            LearnerCourseCatalogError::PermissionDenied(permission) => {
                OrganizationCourseListError::Database(format!(
                    "unexpected permission error while building organization courses: {}",
                    permission
                ))
            }
        }
    }
}

impl From<TeacherCourseDashboardError> for OrganizationCourseListError {
    fn from(error: TeacherCourseDashboardError) -> Self {
        match error {
            TeacherCourseDashboardError::PermissionDenied(permission) => {
                OrganizationCourseListError::Database(format!(
                    "unexpected permission error while building organization courses: {}",
                    permission
                ))
            }
            TeacherCourseDashboardError::NotFound => {
                OrganizationCourseListError::Database("course not found".to_string())
            }
            TeacherCourseDashboardError::Database(message) => {
                OrganizationCourseListError::Database(message)
            }
        }
    }
}
