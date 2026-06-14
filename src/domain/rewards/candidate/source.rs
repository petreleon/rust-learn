use std::fmt;

pub const REWARD_SOURCE_COURSE: &str = "course";
pub const REWARD_SOURCE_ORGANIZATION: &str = "organization";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardCandidateSourceScope {
    Course,
    Organization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceScopeParseError {
    value: String,
}

impl RewardCandidateSourceScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Course => REWARD_SOURCE_COURSE,
            Self::Organization => REWARD_SOURCE_ORGANIZATION,
        }
    }

    pub fn parse(value: &str) -> Result<Self, SourceScopeParseError> {
        match value {
            REWARD_SOURCE_COURSE => Ok(Self::Course),
            REWARD_SOURCE_ORGANIZATION => Ok(Self::Organization),
            other => Err(SourceScopeParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, SourceScopeParseError> {
        let normalized = value.trim().to_ascii_lowercase().replace(['-', ' '], "_");
        Self::parse(&normalized)
    }
}

impl fmt::Display for RewardCandidateSourceScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for SourceScopeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward candidate source scope '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{RewardCandidateSourceScope, REWARD_SOURCE_COURSE};

    #[test]
    fn exposes_stable_source_scope_keys() {
        assert_eq!(
            RewardCandidateSourceScope::Course.as_str(),
            REWARD_SOURCE_COURSE
        );
        assert_eq!(
            RewardCandidateSourceScope::Organization.as_str(),
            "organization"
        );
    }

    #[test]
    fn parses_known_source_scope() {
        assert_eq!(
            RewardCandidateSourceScope::parse("organization").unwrap(),
            RewardCandidateSourceScope::Organization
        );
    }

    #[test]
    fn normalizes_common_source_scope_spellings() {
        assert_eq!(
            RewardCandidateSourceScope::normalize(" organization ").unwrap(),
            RewardCandidateSourceScope::Organization
        );
    }

    #[test]
    fn rejects_unknown_source_scope() {
        assert!(RewardCandidateSourceScope::parse("platform").is_err());
    }
}
