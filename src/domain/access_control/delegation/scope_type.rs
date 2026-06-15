use super::DelegationRuleError;

pub const DELEGATED_SCOPE_COURSE: &str = "course";
pub const DELEGATED_SCOPE_ORGANIZATION: &str = "organization";
pub const DELEGATED_SCOPE_PLATFORM: &str = "platform";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelegatedScopeType {
    Platform,
    Organization,
    Course,
}

impl DelegatedScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Platform => DELEGATED_SCOPE_PLATFORM,
            Self::Organization => DELEGATED_SCOPE_ORGANIZATION,
            Self::Course => DELEGATED_SCOPE_COURSE,
        }
    }

    pub fn parse(scope_type: &str) -> Result<Self, DelegationRuleError> {
        match scope_type {
            DELEGATED_SCOPE_PLATFORM => Ok(Self::Platform),
            DELEGATED_SCOPE_ORGANIZATION => Ok(Self::Organization),
            DELEGATED_SCOPE_COURSE => Ok(Self::Course),
            _ => Err(DelegationRuleError::InvalidInput(
                "unsupported delegated permission scope".to_string(),
            )),
        }
    }

    pub fn normalize(scope_type: &str) -> Result<Self, DelegationRuleError> {
        let normalized = scope_type.trim().to_ascii_lowercase().replace('-', "_");
        Self::parse(&normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_normalizes_delegated_scope_type() {
        assert_eq!(
            DelegatedScopeType::parse(DELEGATED_SCOPE_PLATFORM).unwrap(),
            DelegatedScopeType::Platform
        );
        assert_eq!(
            DelegatedScopeType::normalize("  COURSE  ").unwrap(),
            DelegatedScopeType::Course
        );
        assert_eq!(DelegatedScopeType::Organization.as_str(), "organization");
    }
}
