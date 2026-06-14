use std::fmt;

pub const COURSE_JOIN_STATUS_PENDING: &str = "pending";
pub const COURSE_JOIN_STATUS_WAITLISTED: &str = "waitlisted";
pub const COURSE_JOIN_STATUS_APPROVED: &str = "approved";
pub const COURSE_JOIN_STATUS_REJECTED: &str = "rejected";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseJoinRequestStatus {
    Pending,
    Waitlisted,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseJoinRequestStatusParseError {
    value: String,
}

impl CourseJoinRequestStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => COURSE_JOIN_STATUS_PENDING,
            Self::Waitlisted => COURSE_JOIN_STATUS_WAITLISTED,
            Self::Approved => COURSE_JOIN_STATUS_APPROVED,
            Self::Rejected => COURSE_JOIN_STATUS_REJECTED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, CourseJoinRequestStatusParseError> {
        match value {
            COURSE_JOIN_STATUS_PENDING => Ok(Self::Pending),
            COURSE_JOIN_STATUS_WAITLISTED => Ok(Self::Waitlisted),
            COURSE_JOIN_STATUS_APPROVED => Ok(Self::Approved),
            COURSE_JOIN_STATUS_REJECTED => Ok(Self::Rejected),
            other => Err(CourseJoinRequestStatusParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, CourseJoinRequestStatusParseError> {
        let normalized = value.trim().to_ascii_lowercase();
        Self::parse(&normalized)
    }

    pub fn normalize_decision(value: &str) -> Result<Self, CourseJoinRequestStatusParseError> {
        let status = Self::normalize(value)?;
        if status == Self::Pending {
            return Err(CourseJoinRequestStatusParseError {
                value: value.to_string(),
            });
        }
        Ok(status)
    }
}

impl fmt::Display for CourseJoinRequestStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for CourseJoinRequestStatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown course join status '{}'", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{CourseJoinRequestStatus, COURSE_JOIN_STATUS_WAITLISTED};

    #[test]
    fn exposes_stable_status_keys() {
        assert_eq!(
            CourseJoinRequestStatus::Waitlisted.as_str(),
            COURSE_JOIN_STATUS_WAITLISTED
        );
        assert_eq!(CourseJoinRequestStatus::Pending.as_str(), "pending");
    }

    #[test]
    fn normalizes_known_statuses() {
        assert_eq!(
            CourseJoinRequestStatus::normalize(" APPROVED ").unwrap(),
            CourseJoinRequestStatus::Approved
        );
    }

    #[test]
    fn decision_targets_exclude_pending() {
        assert!(CourseJoinRequestStatus::normalize_decision("pending").is_err());
        assert_eq!(
            CourseJoinRequestStatus::normalize_decision("waitlisted").unwrap(),
            CourseJoinRequestStatus::Waitlisted
        );
    }
}
