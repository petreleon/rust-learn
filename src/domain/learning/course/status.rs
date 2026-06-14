use std::fmt;

pub const COURSE_STATUS_DRAFT: &str = "draft";
pub const COURSE_STATUS_SUBMITTED: &str = "submitted";
pub const COURSE_STATUS_NEEDS_CHANGES: &str = "needs_changes";
pub const COURSE_STATUS_APPROVED: &str = "approved";
pub const COURSE_STATUS_PUBLISHED: &str = "published";
pub const COURSE_STATUS_ARCHIVED: &str = "archived";
pub const COURSE_STATUS_SUSPENDED: &str = "suspended";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseLifecycleStatus {
    Draft,
    Submitted,
    NeedsChanges,
    Approved,
    Published,
    Archived,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseLifecycleStatusParseError {
    value: String,
}

impl CourseLifecycleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => COURSE_STATUS_DRAFT,
            Self::Submitted => COURSE_STATUS_SUBMITTED,
            Self::NeedsChanges => COURSE_STATUS_NEEDS_CHANGES,
            Self::Approved => COURSE_STATUS_APPROVED,
            Self::Published => COURSE_STATUS_PUBLISHED,
            Self::Archived => COURSE_STATUS_ARCHIVED,
            Self::Suspended => COURSE_STATUS_SUSPENDED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, CourseLifecycleStatusParseError> {
        match value {
            COURSE_STATUS_DRAFT => Ok(Self::Draft),
            COURSE_STATUS_SUBMITTED => Ok(Self::Submitted),
            COURSE_STATUS_NEEDS_CHANGES => Ok(Self::NeedsChanges),
            COURSE_STATUS_APPROVED => Ok(Self::Approved),
            COURSE_STATUS_PUBLISHED => Ok(Self::Published),
            COURSE_STATUS_ARCHIVED => Ok(Self::Archived),
            COURSE_STATUS_SUSPENDED => Ok(Self::Suspended),
            other => Err(CourseLifecycleStatusParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, CourseLifecycleStatusParseError> {
        let normalized = value.trim().to_ascii_lowercase();
        Self::parse(&normalized)
    }
}

impl fmt::Display for CourseLifecycleStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for CourseLifecycleStatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown course lifecycle status '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{CourseLifecycleStatus, COURSE_STATUS_PUBLISHED};

    #[test]
    fn exposes_stable_status_keys() {
        assert_eq!(
            CourseLifecycleStatus::Published.as_str(),
            COURSE_STATUS_PUBLISHED
        );
        assert_eq!(
            CourseLifecycleStatus::NeedsChanges.as_str(),
            "needs_changes"
        );
    }

    #[test]
    fn normalizes_known_statuses() {
        assert_eq!(
            CourseLifecycleStatus::normalize(" Published ").unwrap(),
            CourseLifecycleStatus::Published
        );
    }

    #[test]
    fn rejects_unknown_status() {
        assert!(CourseLifecycleStatus::normalize("launched").is_err());
    }
}
