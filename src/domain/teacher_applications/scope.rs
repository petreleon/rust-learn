#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationScopeError {
    InvalidScope(String),
    MissingOrganizationTarget(String),
    MissingCourseTarget(String),
}

pub const TEACHER_APPLICATION_SCOPE_COURSE: &str = "course";
pub const TEACHER_APPLICATION_SCOPE_ORGANIZATION: &str = "organization";
pub const TEACHER_APPLICATION_SCOPE_PLATFORM: &str = "platform";

pub fn normalize_scope(scope: &str) -> Result<String, TeacherApplicationScopeError> {
    let normalized = scope.trim().to_ascii_lowercase();
    match normalized.as_str() {
        TEACHER_APPLICATION_SCOPE_COURSE
        | TEACHER_APPLICATION_SCOPE_ORGANIZATION
        | TEACHER_APPLICATION_SCOPE_PLATFORM => Ok(normalized),
        _ => Err(TeacherApplicationScopeError::InvalidScope(
            "unsupported requested_scope".to_string(),
        )),
    }
}

pub fn validate_requested_scope(
    requested_scope: &str,
    requested_organization_id: Option<i32>,
    requested_course_id: Option<i32>,
    organization_sponsor_id: Option<i32>,
) -> Result<(), TeacherApplicationScopeError> {
    match requested_scope {
        TEACHER_APPLICATION_SCOPE_PLATFORM => Ok(()),
        TEACHER_APPLICATION_SCOPE_ORGANIZATION => {
            if requested_organization_id
                .or(organization_sponsor_id)
                .is_none()
            {
                return Err(TeacherApplicationScopeError::MissingOrganizationTarget(
                    "organization scope requires a requested organization or sponsor".to_string(),
                ));
            }
            Ok(())
        }
        TEACHER_APPLICATION_SCOPE_COURSE => {
            if requested_course_id.is_none() {
                return Err(TeacherApplicationScopeError::MissingCourseTarget(
                    "course scope requires requested_course_id".to_string(),
                ));
            }
            Ok(())
        }
        _ => Err(TeacherApplicationScopeError::InvalidScope(
            "unsupported requested_scope".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_known_scopes() {
        assert_eq!(normalize_scope(" PLATFORM ").unwrap(), "platform");
        assert_eq!(normalize_scope("organization").unwrap(), "organization");
        assert_eq!(normalize_scope("course").unwrap(), "course");
    }

    #[test]
    fn rejects_unknown_scopes() {
        assert!(matches!(
            normalize_scope("school"),
            Err(TeacherApplicationScopeError::InvalidScope(_))
        ));
    }

    #[test]
    fn validates_required_targets() {
        assert!(validate_requested_scope("platform", None, None, None).is_ok());
        assert!(validate_requested_scope("organization", Some(1), None, None).is_ok());
        assert!(validate_requested_scope("organization", None, None, Some(1)).is_ok());
        assert!(validate_requested_scope("course", None, Some(1), None).is_ok());
        assert!(matches!(
            validate_requested_scope("organization", None, None, None),
            Err(TeacherApplicationScopeError::MissingOrganizationTarget(_))
        ));
        assert!(matches!(
            validate_requested_scope("course", None, None, None),
            Err(TeacherApplicationScopeError::MissingCourseTarget(_))
        ));
    }
}
