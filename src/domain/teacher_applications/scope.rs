use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationScopeError {
    InvalidScope(String),
    MissingOrganizationTarget(String),
    MissingCourseTarget(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeacherApplicationScope {
    Course,
    Organization,
    Platform,
}

pub const TEACHER_APPLICATION_SCOPE_COURSE: &str = "course";
pub const TEACHER_APPLICATION_SCOPE_ORGANIZATION: &str = "organization";
pub const TEACHER_APPLICATION_SCOPE_PLATFORM: &str = "platform";

impl TeacherApplicationScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Course => TEACHER_APPLICATION_SCOPE_COURSE,
            Self::Organization => TEACHER_APPLICATION_SCOPE_ORGANIZATION,
            Self::Platform => TEACHER_APPLICATION_SCOPE_PLATFORM,
        }
    }

    pub fn parse(scope: &str) -> Result<Self, TeacherApplicationScopeError> {
        match scope {
            TEACHER_APPLICATION_SCOPE_COURSE => Ok(Self::Course),
            TEACHER_APPLICATION_SCOPE_ORGANIZATION => Ok(Self::Organization),
            TEACHER_APPLICATION_SCOPE_PLATFORM => Ok(Self::Platform),
            _ => Err(TeacherApplicationScopeError::InvalidScope(
                "unsupported requested_scope".to_string(),
            )),
        }
    }

    pub fn normalize(scope: &str) -> Result<Self, TeacherApplicationScopeError> {
        Self::parse(&scope.trim().to_ascii_lowercase())
    }
}

pub fn normalize_scope(scope: &str) -> Result<String, TeacherApplicationScopeError> {
    TeacherApplicationScope::normalize(scope).map(|scope| scope.as_str().to_string())
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

impl fmt::Display for TeacherApplicationScopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidScope(message)
            | Self::MissingOrganizationTarget(message)
            | Self::MissingCourseTarget(message) => formatter.write_str(message),
        }
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
        assert_eq!(
            TeacherApplicationScope::parse("organization").unwrap(),
            TeacherApplicationScope::Organization
        );
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
